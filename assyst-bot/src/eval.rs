use std::pin::Pin;

use crate::command::context::Context;
use anyhow::Context as AnyhowContext;
use dlopen::wrapper::Container;
use dlopen::wrapper::WrapperApi;
use futures::Future;
use lazy_static::lazy_static;
use tokio::fs;
use tokio::process::Command;
use tokio::sync::Mutex;

type HookFn = extern "C" fn(cx: *const Context, out: *mut Option<HookFnOut>);
type HookFnOut = Pin<Box<dyn Future<Output = String> + Send + Sync>>;

#[derive(WrapperApi)]
struct Library {
    assyst_eval_hook: extern "C" fn() -> HookFn,
}

lazy_static! {
    static ref EVAL_LOCK: Mutex<()> = Mutex::new(());
}

pub async fn run(cx: &Context, code: &str) -> anyhow::Result<String> {
    // Concurrent eval compiles are bad
    // So there is a mutex that protects this function from being run concurrently
    let _lock = EVAL_LOCK.lock().await;

    let template = include_str!("evaltemplate.rs");
    let code = template.replace("/* [[CODE]] */", code);
    fs::write("../eval-target/src/lib.rs", code).await?;
    let command = Command::new("bash")
        .arg("-c")
        .arg("cd ../eval-target && cargo build")
        .output()
        .await?;

    if !command.status.success() {
        return Ok(String::from_utf8_lossy(&command.stderr).into_owned());
    }

    let lib = unsafe { Container::<Library>::load("../target/debug/libeval_target.so")? };

    let mut fut = None;

    let fun = lib.assyst_eval_hook();
    fun(cx, &mut fut);

    let fut = fut.context("Future is empty!")?;
    Ok(fut.await)
}
