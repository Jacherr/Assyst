use crate::{assyst::Assyst, util::handle_job_result};
use crate::util::get_wsi_request_tier;
use bincode::{deserialize, serialize};
use bytes::Bytes;
use reqwest::Error;
use serde::Deserialize;
use shared::errors::ProcessingError;
use shared::response_data::{ImageInfo, Stats};
use shared::util::encode_frames;
use shared::{job::JobResult, fifo::{FifoSend, WsiRequest, FifoData}, query_params::*};
use tokio::time::timeout;
use tokio::{sync::{oneshot::{Sender, self}, Mutex, mpsc::UnboundedReceiver}, net::TcpStream, time::sleep, io::{AsyncReadExt, AsyncWriteExt}};
use std::sync::atomic::AtomicBool;
use std::{future::Future, pin::Pin, sync::{Arc, atomic::{Ordering, AtomicUsize}}, collections::HashMap, time::Duration};
use twilight_model::id::UserId;

pub type NoArgFunction = Box<
    dyn Fn(
            Arc<Assyst>,
            Bytes,
            UserId,
        ) -> Pin<Box<dyn Future<Output = Result<Bytes, RequestError>> + Send>>
        + Send
        + Sync,
>;

static CONNECTED: AtomicBool = AtomicBool::new(false);

pub async fn wsi_listen(job_rx: UnboundedReceiver<(Sender<JobResult>, FifoSend, usize)>, socket: &str) {
    let job_rx = Arc::new(Mutex::new(job_rx));

    loop {
        let stream = match TcpStream::connect(socket).await {
            Ok(stream) => stream,
            Err(e) => {
                eprintln!("Failed to connect to WSI: {} (retrying in 30 seconds)", e);
                sleep(Duration::from_secs(30)).await;
                continue;
            }
        };

        println!("Connected to WSI");

        CONNECTED.store(true, Ordering::Relaxed);

        let (mut reader, mut writer) = stream.into_split();
        
        let jobs = Arc::new(Mutex::new(HashMap::<usize, Sender<JobResult>>::new()));
        let jobs_clone = jobs.clone();
    
        let r = tokio::spawn(async move {
            loop {
                let length = match reader.read_u32().await {
                    Err(_) => {
                        break;
                    },
                    Ok(x) => x
                };
    
                let mut buf = vec![0; length as usize];
                if reader.read_exact(&mut buf).await.is_err() {
                    break;
                }
                
                let deserialized = deserialize::<JobResult>(&buf).unwrap();
                let job_id = deserialized.id();
                let mut job_lock = jobs_clone.lock().await;
                let tx = job_lock.remove(&job_id);
                drop(job_lock);
    
                if let Some(tx) = tx {
                    tx.send(deserialized).unwrap();
                }
            }
        });
    
        let job_rx_clone = job_rx.clone();

        let w = tokio::spawn(async move {
            let next_job_id = AtomicUsize::new(0);
    
            loop {
                let (tx, job, premium_level) = match job_rx_clone.lock().await.recv().await {
                    Some(x) => x,
                    None => break
                };
    
                let id = next_job_id.fetch_add(1, Ordering::Relaxed);
    
                let wsi_request = WsiRequest::new(id, premium_level, job);
                let job = serialize(&wsi_request).unwrap();
    
                let mut job_lock = jobs.lock().await;
                job_lock.insert(id, tx);
                drop(job_lock);

                if writer.write_u32(job.len() as u32).await.is_err() {
                    break;
                }
    
                if writer.write_all(&job).await.is_err() {
                    break;
                }
            }
        });
    
        let _ = r.await;
        w.abort();

        CONNECTED.store(false, Ordering::Relaxed);

        eprintln!("Lost connection to WSI server, attempting reconnection in 10 sec...");
        sleep(Duration::from_secs(10)).await;
    }
}

pub async fn run_wsi_job(assyst: Arc<Assyst>, job: FifoSend, user_id: UserId) -> Result<Bytes, RequestError> {
    if !CONNECTED.load(Ordering::Relaxed) {
        return Err(RequestError::Wsi(
            WsiError {
                message: "Assyst cannot establish a connection to the image server at this time. Try again in a few minutes.".to_string().into(),
                code: 0,
            }
        ))
    }

    let premium_level = get_wsi_request_tier(&assyst.clone(), user_id)
        .await
        .map_err(RequestError::Sqlx)?;

    // 3 minute timeout
    const MAX_TIME_LIMIT: Duration = Duration::from_secs(60 * 3);

    let (tx, rx) = oneshot::channel::<JobResult>();
    assyst.send_to_wsi(tx, job, premium_level);

    let res = timeout(MAX_TIME_LIMIT, rx).await;
    let result = match res {
        Err(_) => JobResult::Error((0, ProcessingError::Timeout)),
        Ok(x) => {
            x.unwrap()
        }
    };

    handle_job_result(result)
}

#[derive(Deserialize, Debug)]
pub struct WsiError {
    pub code: u16,
    pub message: Box<str>,
}

#[derive(Debug)]
pub enum RequestError {
    Reqwest(Error),
    Serde(serde_json::Error),
    Wsi(WsiError),
    Sqlx(sqlx::Error),
}
impl From<ProcessingError> for RequestError {
    fn from(e: ProcessingError) -> Self {
        RequestError::Wsi(WsiError {
            code: 0,
            message: e.to_string().into()
        })
    }
}
impl From<crate::rest::annmarie::RequestError> for RequestError {
    fn from(e: crate::rest::annmarie::RequestError) -> Self {
        RequestError::Wsi(WsiError {
            code: 0,
            message: e.to_string().into()
        })
    }
}
impl ToString for RequestError {
    fn to_string(&self) -> String {
        match self {
            RequestError::Reqwest(e) => e.to_string(),
            RequestError::Serde(e) => e.to_string(),
            RequestError::Wsi(e) => e.message.to_string(),
            RequestError::Sqlx(e) => e.to_string(),
        }
    }
}

pub async fn randomize(
    assyst: &Assyst,
    image: Bytes,
    user_id: UserId,
    acceptable_routes: &mut Vec<&'static str>,
) -> (&'static str, Result<Bytes, RequestError>) {
    todo!()

    /* 
    let index = rand::thread_rng().gen_range(0..acceptable_routes.len());
    let route = acceptable_routes.remove(index);

    let bytes = request_bytes(assyst, route, image, &[], user_id).await;

    match bytes {
        Ok(bytes) => (route, Ok(bytes)),
        Err(e) => (route, Err(e)),
    }*/
}

pub async fn _3d_rotate(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::_3dRotate(
        FifoData::new(image.to_vec(), NoneQuery {})
    );

    run_wsi_job(assyst, job, user_id).await
}

pub async fn ahshit(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::AhShit(
        FifoData::new(image.to_vec(), NoneQuery {})
    );

    run_wsi_job(assyst, job, user_id).await
}

pub async fn aprilfools(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::AprilFools(
        FifoData::new(image.to_vec(), NoneQuery {})
    );

    run_wsi_job(assyst, job, user_id).await
}

pub async fn audio(
    assyst: Arc<Assyst>,
    image: Bytes,
    effect: &str,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::Audio(
        FifoData::new(image.to_vec(), AudioQueryParams { effect: effect.to_string() })
    );

    run_wsi_job(assyst, job, user_id).await
}

pub async fn blur(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
    power: &str,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::Blur(
        FifoData::new(image.to_vec(), BlurQueryParams { power: power.parse::<f32>().unwrap() })
    );

    run_wsi_job(assyst, job, user_id).await
}

pub async fn caption(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
    text: &str,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::Caption(
        FifoData::new(image.to_vec(), CaptionQueryParams { text: text.to_string() })
    );

    run_wsi_job(assyst, job, user_id).await
}

pub async fn convert_png(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::ConvertPng(
        FifoData::new(image.to_vec(), NoneQuery {})
    );

    run_wsi_job(assyst, job, user_id).await
}

pub async fn fix_transparency(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::FixTransparency(
        FifoData::new(image.to_vec(), NoneQuery {})
    );

    run_wsi_job(assyst, job, user_id).await
}

pub async fn flash(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::Flash(
        FifoData::new(image.to_vec(), NoneQuery {})
    );

    run_wsi_job(assyst, job, user_id).await
}
pub async fn flip(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::Flip(
        FifoData::new(image.to_vec(), NoneQuery {})
    );

    run_wsi_job(assyst, job, user_id).await
}

pub async fn flop(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::Flop(
        FifoData::new(image.to_vec(), NoneQuery {})
    );

    run_wsi_job(assyst, job, user_id).await
}

pub async fn motivate(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
    top_text: &str,
    bottom_text: &str,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::Motivate(
        FifoData::new(image.to_vec(), MotivateQueryParams {
            top: top_text.to_string(),
            bottom: Some(bottom_text.to_string()),
        })
    );

    run_wsi_job(assyst, job, user_id).await
}

pub async fn frames(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::Frames(
        FifoData::new(image.to_vec(), NoneQuery {})
    );

    run_wsi_job(assyst, job, user_id).await
}

pub async fn ghost(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
    depth: &str,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::Ghost(
        FifoData::new(image.to_vec(), GhostQueryParams {
            depth: Some(depth.parse::<usize>().unwrap()),
        })
    );

    run_wsi_job(assyst, job, user_id).await
}

pub async fn gif_loop(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::GifLoop(
        FifoData::new(image.to_vec(), NoneQuery {})
    );

    run_wsi_job(assyst, job, user_id).await
}

pub async fn gif_magik(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::GifMagik(
        FifoData::new(image.to_vec(), NoneQuery {})
    );

    run_wsi_job(assyst, job, user_id).await
}

pub async fn gif_scramble(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::GifScramble(
        FifoData::new(image.to_vec(), NoneQuery {})
    );

    run_wsi_job(assyst, job, user_id).await
}

pub async fn grayscale(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::Grayscale(
        FifoData::new(image.to_vec(), NoneQuery {})
    );

    run_wsi_job(assyst, job, user_id).await
}

pub async fn heart_locket(
    assyst: Arc<Assyst>,
    image: Bytes,
    text: &str,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    let text_job = FifoSend::HeartLocketText(
        FifoData::new(vec![], HeartLocketTextQueryParams { text: text.to_string() })
    );
    let text = run_wsi_job(assyst.clone(), text_job, user_id).await?;

    let resized = FifoSend::Resize(
        FifoData::new(image.to_vec(), ResizeQueryParams {
            width: Some(250),
            height: Some(250),
            method: Some(ResizeMethod::Nearest),
            scale: None
        })
    );
    let resized = run_wsi_job(assyst.clone(), resized, user_id).await?;

    let ann_buffer = encode_frames(vec![text.to_vec(), resized.to_vec()]);

    let gif_job = FifoSend::ConstructGif(
        FifoData::new(ann_buffer, ConstructGifQueryParams {
            delays: vec![],
            repeat: -1,
            audio: None
        })
    );
    let gif = run_wsi_job(assyst.clone(), gif_job, user_id).await?;

    let job = FifoSend::Annmarie(
        FifoData::new(gif.to_vec(), AnnmarieQueryParams { 
            route: crate::rest::annmarie::routes::MAKESWEET.to_string(),
            query_params: vec![
                ("template".to_string(), "heart-locket".to_string()),
            ],
            images: vec![],
            preprocess: true
         })
    );

    run_wsi_job(assyst, job, user_id).await
}

pub async fn gif_speed(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
    delay: Option<&str>,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::GifSpeed(
        FifoData::new(image.to_vec(), GifSpeedQueryParams {
            delay: delay.map(|s| s.parse::<usize>().unwrap()),
        })
    );

    run_wsi_job(assyst, job, user_id).await
}

pub async fn image_info(assyst: Arc<Assyst>, image: Bytes) -> Result<ImageInfo, RequestError> {
    let job = FifoSend::ImageInfo(
        FifoData::new(image.to_vec(), NoneQuery {})
    );

    let result = run_wsi_job(assyst, job, UserId::from(0)).await?;
    let v = result.to_vec();
    let de = deserialize::<ImageInfo>(&v).unwrap().clone();

    Ok(de)
}

pub async fn imagemagick_eval(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
    script: &str,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::ImageMagickEval(
        FifoData::new(image.to_vec(), ImageMagickEvalQueryParams {
            script: script.to_string()
        })
    );

    run_wsi_job(assyst, job, user_id).await
}

pub async fn invert(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::Invert(
        FifoData::new(image.to_vec(), NoneQuery {})
    );

    run_wsi_job(assyst, job, user_id).await
}

pub async fn jpeg(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::Jpeg(
        FifoData::new(image.to_vec(), NoneQuery {})
    );

    run_wsi_job(assyst, job, user_id).await
}

pub async fn magik(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::Magik(
        FifoData::new(image.to_vec(), NoneQuery {})
    );

    run_wsi_job(assyst, job, user_id).await
}

pub async fn meme(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
    top: &str,
    bottom: &str,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::Meme(
        FifoData::new(image.to_vec(), MemeQueryParams {
            top: top.to_string(),
            bottom: Some(bottom.to_string()),
        })
    );

    run_wsi_job(assyst, job, user_id).await
}

pub async fn pixelate(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
    downscaled_height: Option<&str>,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::Pixelate(
        FifoData::new(image.to_vec(), PixelateQueryParams {
            downscaled_height: downscaled_height.map(|s| s.parse::<usize>().unwrap()),
        })
    );

    run_wsi_job(assyst, job, user_id).await
}

pub async fn preprocess(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::Preprocess(
        FifoData::new(image.to_vec(), NoneQuery {})
    );

    run_wsi_job(assyst, job, user_id).await
}

pub async fn printer(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::Printer(
        FifoData::new(image.to_vec(), NoneQuery {})
    );

    run_wsi_job(assyst, job, user_id).await
}

pub async fn rainbow(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::Rainbow(
        FifoData::new(image.to_vec(), NoneQuery {})
    );

    run_wsi_job(assyst, job, user_id).await
}

pub async fn resize(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
    method: ResizeMethod,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::Resize(
        FifoData::new(image.to_vec(), ResizeQueryParams {
            method: Some(method),
            width: None,
            height: None,
            scale: None
        })
    );

    run_wsi_job(assyst, job, user_id).await
}

pub async fn resize_scale(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
    scale: f32,
    method: ResizeMethod,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::Resize(
        FifoData::new(image.to_vec(), ResizeQueryParams {
            method: Some(method),
            width: None,
            height: None,
            scale: Some(scale)
        })
    );

    run_wsi_job(assyst, job, user_id).await
}

pub async fn resize_width_height(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
    width: usize,
    height: usize,
    method: ResizeMethod,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::Resize(
        FifoData::new(image.to_vec(), ResizeQueryParams {
            method: Some(method),
            width: Some(width as u32),
            height: Some(height as u32),
            scale: None
        })
    );

    run_wsi_job(assyst, job, user_id).await
}

pub async fn overlay(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
    overlay: &str,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::Overlay(
        FifoData::new(image.to_vec(), OverlayQueryParams {
            overlay: overlay.to_string(),
        })
    );

    run_wsi_job(assyst, job, user_id).await
}

pub async fn reverse(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::Reverse(
        FifoData::new(image.to_vec(), NoneQuery {})
    );

    run_wsi_job(assyst, job, user_id).await
}

pub async fn rotate(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
    degrees: &str,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::Rotate(
        FifoData::new(image.to_vec(), RotateQueryParams {
            degrees: degrees.parse::<usize>().unwrap(),
        })
    );

    run_wsi_job(assyst, job, user_id).await
}

pub async fn set_loop(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
    looping: bool,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::SetLoop(
        FifoData::new(image.to_vec(), SetLoopQueryParams {
            r#loop: looping,
        })
    );

    run_wsi_job(assyst, job, user_id).await
}

pub async fn spin(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::Spin(
        FifoData::new(image.to_vec(), NoneQuery {})
    );

    run_wsi_job(assyst, job, user_id).await
}

pub async fn spread(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::Spread(
        FifoData::new(image.to_vec(), NoneQuery {})
    );

    run_wsi_job(assyst, job, user_id).await
}

pub async fn stats(assyst: Arc<Assyst>) -> Result<Stats, RequestError> {
    let job = FifoSend::Stats(
        FifoData::new(vec![], NoneQuery {})
    );

    let result = run_wsi_job(assyst, job, UserId::from(0)).await?;
    Ok(deserialize::<Stats>(&result).unwrap())
}

pub async fn swirl(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::Swirl(
        FifoData::new(image.to_vec(), NoneQuery {})
    );

    run_wsi_job(assyst, job, user_id).await
}

pub async fn tehi(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::Tehi(
        FifoData::new(image.to_vec(), NoneQuery {})
    );

    run_wsi_job(assyst, job, user_id).await
}

pub async fn wall(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::Wall(
        FifoData::new(image.to_vec(), NoneQuery {})
    );

    run_wsi_job(assyst, job, user_id).await
}

pub async fn wave(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::Wave(
        FifoData::new(image.to_vec(), NoneQuery {})
    );

    run_wsi_job(assyst, job, user_id).await
}

pub async fn wormhole(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::Wormhole(
        FifoData::new(image.to_vec(), NoneQuery {})
    );

    run_wsi_job(assyst, job, user_id).await
}

pub async fn zoom(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::Zoom(
        FifoData::new(image.to_vec(), NoneQuery {})
    );

    run_wsi_job(assyst, job, user_id).await
}

pub fn format_err(err: RequestError) -> String {
    match err {
        RequestError::Reqwest(_) => String::from("A network error occurred"),
        RequestError::Wsi(e) => e.message.to_string(),
        RequestError::Serde(e) => e.to_string(),
        RequestError::Sqlx(e) => e.to_string(),
    }
}