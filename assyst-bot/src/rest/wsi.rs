use crate::util::get_wsi_request_tier;
use crate::{assyst::Assyst, util::handle_job_result};
use assyst_common::util::UserId;
use bincode::{deserialize, serialize};
use bytes::Bytes;
use reqwest::Error;
use serde::Deserialize;
use shared::errors::ProcessingError;
use shared::response_data::{ImageInfo, Stats};
use shared::{
    fifo::{FifoData, FifoSend, WsiRequest},
    job::JobResult,
    query_params::*,
};
use std::sync::atomic::AtomicBool;
use std::{
    collections::HashMap,
    future::Future,
    pin::Pin,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    time::Duration,
};
use tokio::time::timeout;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    sync::{
        mpsc::UnboundedReceiver,
        oneshot::{self, Sender},
        Mutex,
    },
    time::sleep,
};

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

pub async fn wsi_listen(
    job_rx: UnboundedReceiver<(Sender<JobResult>, FifoSend, usize)>,
    socket: &str,
) {
    let job_rx = Arc::new(Mutex::new(job_rx));

    loop {
        let stream = match TcpStream::connect(socket).await {
            Ok(stream) => stream,
            Err(_) => {
                sleep(Duration::from_secs(10)).await;
                continue;
            }
        };

        println!("Connected to WSI");

        CONNECTED.store(true, Ordering::Relaxed);

        let (mut reader, mut writer) = stream.into_split();

        let jobs = Arc::new(Mutex::new(HashMap::<usize, Sender<JobResult>>::new()));
        let jobs_clone = jobs.clone();
        let jobs_clone_2 = jobs.clone();

        let mut r = tokio::spawn(async move {
            loop {
                println!("Awaiting WSI read...");
                let length = match reader.read_u32().await {
                    Err(e) => {
                        eprintln!("Failed to read length from WSI: {:?}", e);
                        break;
                    }
                    Ok(x) => x,
                };
                println!("Read length, waiting for data...");
                let mut buf = vec![0; length as usize];
                match reader.read_exact(&mut buf).await {
                    Err(e) => {
                        eprintln!("Failed to read buffer from WSI: {:?}", e);
                        break;
                    }
                    _ => {}
                }
                println!("Read data, decoding...");

                let deserialized = match deserialize::<JobResult>(&buf) {
                    Ok(x) => x,
                    Err(e) => {
                        eprintln!("Failed to deserialize WSI data: {:?}", e);
                        continue;
                    }
                };

                println!("got id: {}", deserialized.id());

                let job_id = deserialized.id();
                let tx = jobs_clone.lock().await.remove(&job_id);

                if let Some(tx) = tx {
                    // if this fails it means it timed out
                    let res = tx.send(deserialized);
                    if res.is_err() {
                        eprintln!("Failed to send job result ID {} to job sender", job_id);
                    } else {
                        println!("Sent job result ID {} to job sender", job_id);
                    }
                }
            }
        });

        let job_rx_clone = job_rx.clone();

        let mut w = tokio::spawn(async move {
            let next_job_id = AtomicUsize::new(0);

            loop {
                let (tx, job, premium_level) = match job_rx_clone.lock().await.recv().await {
                    Some(x) => x,
                    None => break,
                };

                let id = next_job_id.fetch_add(1, Ordering::Relaxed);

                println!("process id: {}", id);

                let wsi_request = WsiRequest::new(id, premium_level, job);
                let job = serialize(&wsi_request).unwrap();

                jobs.lock().await.insert(id, tx);

                println!("sending id: {}", id);
                match writer.write_u32(job.len() as u32).await {
                    Err(e) => {
                        println!("Failed to write to WSI: {:?}", e.to_string());
                        break;
                    }
                    _ => {}
                }

                println!("sending data for: {}", id);
                match writer.write_all(&job).await {
                    Err(e) => {
                        println!("Failed to write to WSI: {:?}", e.to_string());
                        break;
                    }
                    _ => {}
                }

                println!("sent data for: {}", id);
            }
        });

        let _ = tokio::select! {
            output = &mut r => { w.abort(); output },
            output = &mut w => { r.abort(); output },
        };

        CONNECTED.store(false, Ordering::Relaxed);
        let mut lock = jobs_clone_2.lock().await;
        let keys = lock.keys().map(|x| *x).collect::<Vec<_>>().clone();

        for job in keys {
            let tx = lock.remove(&job).unwrap();
            let _ = tx.send(JobResult::new_err(
                0,
                ProcessingError::Other(
                    "The image server died. Try again in a few seconds.".to_owned(),
                ),
            ));
        }

        eprintln!("Lost connection to WSI server, attempting reconnection in 10 sec...");
        sleep(Duration::from_secs(10)).await;
    }
}

pub async fn run_wsi_job(
    assyst: Arc<Assyst>,
    job: FifoSend,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    if !CONNECTED.load(Ordering::Relaxed) {
        return Err(RequestError::Wsi(
            WsiError {
                message: "Assyst cannot establish a connection to the image server at this time. Try again in a few minutes.".to_string().into(),
                code: 0,
            }
        ));
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
        Err(_) => JobResult::new_err(0, ProcessingError::Timeout),
        Ok(x) => x.unwrap_or(JobResult::new_err(
            0,
            ProcessingError::Other(
                "The image server died. Try again in a couple of minutes.".to_string(),
            ),
        )),
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
            message: e.to_string().into(),
        })
    }
}

impl std::fmt::Display for RequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RequestError::Reqwest(_) => write!(f, "A network error occurred"),
            RequestError::Serde(e) => write!(f, "{}", e),
            RequestError::Wsi(e) => write!(f, "{}", e.message),
            RequestError::Sqlx(e) => write!(f, "{}", e),
        }
    }
}

impl std::error::Error for RequestError {}

pub async fn randomize(
    _assyst: &Assyst,
    _image: Bytes,
    _user_id: UserId,
    _acceptable_routes: &mut Vec<&'static str>,
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
    let job = FifoSend::_3dRotate(FifoData::new(image.to_vec(), NoneQuery {}));

    run_wsi_job(assyst, job, user_id).await
}

pub async fn ahshit(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::AhShit(FifoData::new(image.to_vec(), NoneQuery {}));

    run_wsi_job(assyst, job, user_id).await
}

pub async fn aprilfools(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::AprilFools(FifoData::new(image.to_vec(), NoneQuery {}));

    run_wsi_job(assyst, job, user_id).await
}

pub async fn audio(
    assyst: Arc<Assyst>,
    image: Bytes,
    effect: &str,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::Audio(FifoData::new(
        image.to_vec(),
        AudioQueryParams {
            effect: effect.to_string(),
        },
    ));

    run_wsi_job(assyst, job, user_id).await
}

pub async fn back_tattoo(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::Makesweet(FifoData::new(
        vec![],
        MakesweetQueryParams {
            template: "back-tattoo".to_string(),
            images: vec![image.to_vec()],
        },
    ));

    run_wsi_job(assyst, job, user_id).await
}

pub async fn book(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::Makesweet(FifoData::new(
        vec![],
        MakesweetQueryParams {
            template: "book".to_string(),
            images: vec![image.to_vec()],
        },
    ));

    run_wsi_job(assyst, job, user_id).await
}

pub async fn billboard(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::Makesweet(FifoData::new(
        vec![],
        MakesweetQueryParams {
            template: "billboard-cityscape".to_string(),
            images: vec![image.to_vec()],
        },
    ));

    run_wsi_job(assyst, job, user_id).await
}

pub async fn blur(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
    power: &str,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::Blur(FifoData::new(
        image.to_vec(),
        BlurQueryParams {
            power: power.parse::<f32>().unwrap_or(1.0),
        },
    ));

    run_wsi_job(assyst, job, user_id).await
}

pub async fn bloom(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
    radius: usize,
    brightness: usize,
    sharpness: usize,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::Bloom(FifoData::new(
        image.to_vec(),
        BloomQueryParams {
            radius,
            brightness,
            sharpness,
        },
    ));

    run_wsi_job(assyst, job, user_id).await
}

pub async fn caption(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
    text: &str,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::Caption(FifoData::new(
        image.to_vec(),
        CaptionQueryParams {
            text: text.to_string(),
        },
    ));

    run_wsi_job(assyst, job, user_id).await
}

pub async fn circuitboard(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::Makesweet(FifoData::new(
        vec![],
        MakesweetQueryParams {
            template: "circuitboard".to_string(),
            images: vec![image.to_vec()],
        },
    ));

    run_wsi_job(assyst, job, user_id).await
}

pub async fn convert_png(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::ConvertPng(FifoData::new(image.to_vec(), NoneQuery {}));

    run_wsi_job(assyst, job, user_id).await
}

pub async fn fisheye(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::FishEye(FifoData::new(image.to_vec(), NoneQuery {}));

    run_wsi_job(assyst, job, user_id).await
}

pub async fn fix_transparency(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::FixTransparency(FifoData::new(image.to_vec(), NoneQuery {}));

    run_wsi_job(assyst, job, user_id).await
}

pub async fn flash(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::Flash(FifoData::new(image.to_vec(), NoneQuery {}));

    run_wsi_job(assyst, job, user_id).await
}

pub async fn flip(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::Flip(FifoData::new(image.to_vec(), NoneQuery {}));

    run_wsi_job(assyst, job, user_id).await
}

pub async fn flop(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::Flop(FifoData::new(image.to_vec(), NoneQuery {}));

    run_wsi_job(assyst, job, user_id).await
}

pub async fn motivate(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
    top_text: &str,
    bottom_text: &str,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::Motivate(FifoData::new(
        image.to_vec(),
        MotivateQueryParams {
            top: top_text.to_string(),
            bottom: Some(bottom_text.to_string()),
        },
    ));

    run_wsi_job(assyst, job, user_id).await
}

pub async fn flag(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::Makesweet(FifoData::new(
        vec![],
        MakesweetQueryParams {
            template: "flag".to_string(),
            images: vec![image.to_vec()],
        },
    ));

    run_wsi_job(assyst, job, user_id).await
}

pub async fn flag2(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::Makesweet(FifoData::new(
        vec![],
        MakesweetQueryParams {
            template: "flag2".to_string(),
            images: vec![image.to_vec()],
        },
    ));

    run_wsi_job(assyst, job, user_id).await
}

pub async fn frames(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::Frames(FifoData::new(image.to_vec(), NoneQuery {}));

    run_wsi_job(assyst, job, user_id).await
}

pub async fn fortune_cookie(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::Makesweet(FifoData::new(
        vec![],
        MakesweetQueryParams {
            template: "fortune-cookie".to_string(),
            images: vec![image.to_vec()],
        },
    ));

    run_wsi_job(assyst, job, user_id).await
}

pub async fn ghost(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
    depth: &str,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::Ghost(FifoData::new(
        image.to_vec(),
        GhostQueryParams {
            depth: Some(depth.parse::<usize>().unwrap_or(5)),
        },
    ));

    run_wsi_job(assyst, job, user_id).await
}

pub async fn gif_loop(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::GifLoop(FifoData::new(image.to_vec(), NoneQuery {}));

    run_wsi_job(assyst, job, user_id).await
}

pub async fn gif_magik(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::GifMagik(FifoData::new(image.to_vec(), NoneQuery {}));

    run_wsi_job(assyst, job, user_id).await
}

pub async fn gif_scramble(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::GifScramble(FifoData::new(image.to_vec(), NoneQuery {}));

    run_wsi_job(assyst, job, user_id).await
}

pub async fn grayscale(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::Grayscale(FifoData::new(image.to_vec(), NoneQuery {}));

    run_wsi_job(assyst, job, user_id).await
}

pub async fn heart_locket(
    assyst: Arc<Assyst>,
    image: Bytes,
    text: &str,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    let text_job = FifoSend::HeartLocketText(FifoData::new(
        vec![],
        HeartLocketTextQueryParams {
            text: text.to_string(),
        },
    ));
    let text = run_wsi_job(assyst.clone(), text_job, user_id).await?;

    let resized = FifoSend::Resize(FifoData::new(
        image.to_vec(),
        ResizeQueryParams {
            width: Some(250),
            height: Some(250),
            method: Some(ResizeMethod::Nearest),
            scale: None,
        },
    ));
    let resized = run_wsi_job(assyst.clone(), resized, user_id).await?;

    let heartlocket = FifoSend::Makesweet(FifoData::new(
        resized.to_vec(),
        MakesweetQueryParams {
            template: "heart-locket".to_string(),
            images: vec![image.to_vec(), text.to_vec()],
        },
    ));

    run_wsi_job(assyst, heartlocket, user_id).await
}

pub async fn gif_speed(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
    delay: Option<&str>,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::GifSpeed(FifoData::new(
        image.to_vec(),
        GifSpeedQueryParams {
            delay: delay.map(|s| s.parse::<usize>().unwrap_or(2)),
        },
    ));

    run_wsi_job(assyst, job, user_id).await
}

pub async fn image_info(assyst: Arc<Assyst>, image: Bytes) -> Result<ImageInfo, RequestError> {
    let job = FifoSend::ImageInfo(FifoData::new(image.to_vec(), NoneQuery {}));

    let result = run_wsi_job(assyst, job, UserId::new(1)).await?;
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
    let job = FifoSend::ImageMagickEval(FifoData::new(
        image.to_vec(),
        ImageMagickEvalQueryParams {
            script: script.to_string(),
        },
    ));

    run_wsi_job(assyst, job, user_id).await
}

pub async fn invert(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::Invert(FifoData::new(image.to_vec(), NoneQuery {}));

    run_wsi_job(assyst, job, user_id).await
}

pub async fn frame_shift(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::FrameShift(FifoData::new(image.to_vec(), NoneQuery {}));

    run_wsi_job(assyst, job, user_id).await
}

pub async fn jpeg(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::Jpeg(FifoData::new(image.to_vec(), NoneQuery {}));

    run_wsi_job(assyst, job, user_id).await
}

pub async fn globe(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::Globe(FifoData::new(image.to_vec(), NoneQuery {}));

    run_wsi_job(assyst, job, user_id).await
}

pub async fn magik(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::Magik(FifoData::new(image.to_vec(), NoneQuery {}));

    run_wsi_job(assyst, job, user_id).await
}

pub async fn meme(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
    top: &str,
    bottom: &str,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::Meme(FifoData::new(
        image.to_vec(),
        MemeQueryParams {
            top: top.to_string(),
            bottom: Some(bottom.to_string()),
        },
    ));

    run_wsi_job(assyst, job, user_id).await
}

pub async fn neon(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
    radius: usize,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::Neon(FifoData::new(image.to_vec(), NeonQueryParams { radius }));

    run_wsi_job(assyst, job, user_id).await
}

pub async fn paint(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::Paint(FifoData::new(image.to_vec(), NoneQuery {}));

    run_wsi_job(assyst, job, user_id).await
}

pub async fn pixelate(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
    downscaled_height: Option<usize>,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::Pixelate(FifoData::new(
        image.to_vec(),
        PixelateQueryParams { downscaled_height },
    ));

    run_wsi_job(assyst, job, user_id).await
}

pub async fn printer(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::Printer(FifoData::new(image.to_vec(), NoneQuery {}));

    run_wsi_job(assyst, job, user_id).await
}

pub async fn rainbow(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::Rainbow(FifoData::new(image.to_vec(), NoneQuery {}));

    run_wsi_job(assyst, job, user_id).await
}

pub async fn resize(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
    method: ResizeMethod,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::Resize(FifoData::new(
        image.to_vec(),
        ResizeQueryParams {
            method: Some(method),
            width: None,
            height: None,
            scale: None,
        },
    ));

    run_wsi_job(assyst, job, user_id).await
}

pub async fn resize_scale(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
    scale: f32,
    method: ResizeMethod,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::Resize(FifoData::new(
        image.to_vec(),
        ResizeQueryParams {
            method: Some(method),
            width: None,
            height: None,
            scale: Some(scale),
        },
    ));

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
    let job = FifoSend::Resize(FifoData::new(
        image.to_vec(),
        ResizeQueryParams {
            method: Some(method),
            width: Some(width as u32),
            height: Some(height as u32),
            scale: None,
        },
    ));

    run_wsi_job(assyst, job, user_id).await
}

pub async fn overlay(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
    overlay: &str,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::Overlay(FifoData::new(
        image.to_vec(),
        OverlayQueryParams {
            overlay: overlay.to_string(),
        },
    ));

    run_wsi_job(assyst, job, user_id).await
}

pub async fn reverse(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::Reverse(FifoData::new(image.to_vec(), NoneQuery {}));

    run_wsi_job(assyst, job, user_id).await
}

pub async fn rotate(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
    degrees: &str,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::Rotate(FifoData::new(
        image.to_vec(),
        RotateQueryParams {
            degrees: degrees.parse::<usize>().unwrap_or(90),
        },
    ));

    run_wsi_job(assyst, job, user_id).await
}

pub async fn rubiks(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::Makesweet(FifoData::new(
        vec![],
        MakesweetQueryParams {
            template: "rubix".to_string(),
            images: vec![
                image.to_vec(),
                image.to_vec(),
                image.to_vec(),
                image.to_vec(),
                image.to_vec(),
                image.to_vec(),
            ],
        },
    ));

    run_wsi_job(assyst, job, user_id).await
}

pub async fn set_loop(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
    looping: bool,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::SetLoop(FifoData::new(
        image.to_vec(),
        SetLoopQueryParams { r#loop: looping },
    ));

    run_wsi_job(assyst, job, user_id).await
}

pub async fn speechbubble(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::SpeechBubble(FifoData::new(image.to_vec(), NoneQuery {}));

    run_wsi_job(assyst, job, user_id).await
}

pub async fn spin(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::Spin(FifoData::new(image.to_vec(), NoneQuery {}));

    run_wsi_job(assyst, job, user_id).await
}

pub async fn spread(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::Spread(FifoData::new(image.to_vec(), NoneQuery {}));

    run_wsi_job(assyst, job, user_id).await
}

pub async fn stats(assyst: Arc<Assyst>) -> Result<Stats, RequestError> {
    let job = FifoSend::Stats(FifoData::new(vec![], NoneQuery {}));

    let result = run_wsi_job(assyst, job, UserId::new(1)).await?;
    Ok(deserialize::<Stats>(&result).unwrap())
}

pub async fn swirl(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::Swirl(FifoData::new(image.to_vec(), NoneQuery {}));

    run_wsi_job(assyst, job, user_id).await
}

pub async fn tehi(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::Tehi(FifoData::new(image.to_vec(), NoneQuery {}));

    run_wsi_job(assyst, job, user_id).await
}

pub async fn toaster(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::Makesweet(FifoData::new(
        vec![],
        MakesweetQueryParams {
            template: "toaster".to_string(),
            images: vec![image.to_vec(), image.to_vec()],
        },
    ));

    run_wsi_job(assyst, job, user_id).await
}

pub async fn uncaption(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::Uncaption(FifoData::new(image.to_vec(), NoneQuery {}));

    run_wsi_job(assyst, job, user_id).await
}

pub async fn valentine(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::Makesweet(FifoData::new(
        vec![],
        MakesweetQueryParams {
            template: "valentine".to_string(),
            images: vec![image.to_vec()],
        },
    ));

    run_wsi_job(assyst, job, user_id).await
}

pub async fn videotogif(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::VideoToGif(FifoData::new(image.to_vec(), NoneQuery {}));

    run_wsi_job(assyst, job, user_id).await
}

pub async fn wall(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::Wall(FifoData::new(image.to_vec(), NoneQuery {}));

    run_wsi_job(assyst, job, user_id).await
}

pub async fn wave(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::Wave(FifoData::new(image.to_vec(), NoneQuery {}));

    run_wsi_job(assyst, job, user_id).await
}

pub async fn wormhole(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::Wormhole(FifoData::new(image.to_vec(), NoneQuery {}));

    run_wsi_job(assyst, job, user_id).await
}

pub async fn zoom(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::Zoom(FifoData::new(image.to_vec(), NoneQuery {}));

    run_wsi_job(assyst, job, user_id).await
}

pub async fn zoom_blur(
    assyst: Arc<Assyst>,
    image: Bytes,
    user_id: UserId,
    factor: f64,
) -> Result<Bytes, RequestError> {
    let job = FifoSend::ZoomBlur(FifoData::new(
        image.to_vec(),
        ZoomBlurQueryParams { factor },
    ));

    run_wsi_job(assyst, job, user_id).await
}
