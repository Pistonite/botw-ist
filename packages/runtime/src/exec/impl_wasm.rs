use wasm_bindgen_spawn::{JoinHandle, ThreadCreator};

use crate::exec::{self, Error, JobSender, Join, Spawn};

pub struct Spawner {
    creator: ThreadCreator,
}
impl Spawner {
    /// Create new spawner using WASM shared-memory-based WebWorker threads
    pub async fn try_new(
        wasm_module_path: &str,
        wasm_bindgen_js_path: &str,
    ) -> Result<Self, Error> {
        log::info!("creating spawner");
        let creator = match ThreadCreator::unready(wasm_module_path, wasm_bindgen_js_path) {
            Err(e) => {
                log::error!("failed to create spawner!");
                // we have to use web_sys to log JsValue errors
                web_sys::console::error_1(&e);
                // then we return an opaque error
                return Err(Error::CreateSpawner);
            }
            Ok(x) => x,
        };
        log::info!("waiting for spawner to be ready");
        let creator = match creator.ready().await {
            Err(e) => {
                log::error!("failed to wait for spawner to be ready!");
                web_sys::console::error_1(&e);
                return Err(Error::CreateSpawner);
            }
            Ok(x) => x,
        };
        Ok(Self { creator })
    }
}

impl Spawn for Spawner {
    type Joiner = JoinHandle<()>;

    fn spawn(&mut self, slot: usize) -> Result<(Self::Joiner, JobSender), Error> {
        let (thread, handle) = exec::make_thread(slot, Default::default());
        // block until Processor is fixed to be Send
        let join_handle = self
            .creator
            .spawn(move || {
                thread.main_loop();
            })
            .map_err(|e| {
                log::error!("failed to create thread: {e}");
                Error::CreateThread(e.to_string())
            })?;
        Ok((join_handle, handle))
    }
}

impl Join for JoinHandle<()> {
    fn join(self) -> Result<(), Error> {
        self.join()
            .map_err(|_| Error::Join("failed to join thread".to_string()))
    }
}
