use deno_core::{JsRuntime, PollEventLoopOptions, RuntimeOptions};
use tokio_util::sync::{CancellationToken, DropGuard};

pub struct EmbeddedRuntime {
    drop_guard: DropGuard,
}

impl EmbeddedRuntime {
    pub fn new() -> Self {
        let runtime_options = RuntimeOptions {
            extensions: vec![],
            ..Default::default()
        };
        let cancel_token = CancellationToken::new();
        let guard = cancel_token.clone().drop_guard();
        let mut runtime = JsRuntime::new(runtime_options);
        let mut poll_options = PollEventLoopOptions::default();
        poll_options.wait_for_inspector = false;
        let cancel_fut = cancel_token.cancelled_owned();


        todo!()
    }
}