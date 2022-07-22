#![allow(unused)]

use std::any::Any;
use std::fmt::Debug;

use assyst::command::context::Context;

mod __priv {
    use std::future::Future;
    use std::pin::Pin;

    use assyst::command::context::Context;

    type AsyncFnOut = Pin<Box<dyn Future<Output = String> + Send + Sync>>;
    type AsyncFn = extern "C" fn(*const Context, *mut Option<AsyncFnOut>);

    #[no_mangle]
    extern "C" fn assyst_eval_hook() -> AsyncFn {
        extern "C" fn out(context: *const Context, out: *mut Option<AsyncFnOut>) {
            let res = Box::pin(super::run(unsafe { &*context }));
            unsafe { out.write(Some(res)) };
        }
        out
    }
}

trait Formattable {
    fn fmt_string(&self) -> String;
}

impl<T: Debug + 'static> Formattable for T {
    fn fmt_string(&self) -> String {
        let t = self as &dyn Any;
        if let Some(s) = t.downcast_ref::<String>() {
            s.clone()
        } else if let Some(s) = t.downcast_ref::<&str>() {
            s.to_string()
        } else {
            format!("{:?}", self)
        }
    }
}

async fn run(context: &Context) -> String {
    Formattable::fmt_string(&{ /* [[CODE]] */ })
}
