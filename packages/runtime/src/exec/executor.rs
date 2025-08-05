use std::sync::{
    Mutex,
    atomic::{AtomicU32, AtomicUsize, Ordering},
};

use blueflame::processor::Cpu1;

use crate::exec::{Error, JobSender, Join, Spawn};

/// A non-work-stealing executor pool of emulated processors
///
/// When thread dies, it spawns new threads to replace the dead ones.
pub struct ExecutorImpl<S: Spawn> {
    /// Spawner to spawn new threads
    spawner: Mutex<S>,
    /// Handles to the spawned threads
    handles: Mutex<Vec<(S::Joiner, JobSender)>>,
    handles_len: AtomicUsize,
    /// Serial number to round-robin the threads
    serial: AtomicU32,
}

impl<S: Spawn> ExecutorImpl<S> {
    /// Create a new executor. Note this does not spawn any threads yet
    pub fn new(spawner: S) -> Self {
        ExecutorImpl {
            spawner: Mutex::new(spawner),
            handles: Mutex::new(Vec::new()),
            handles_len: AtomicUsize::new(0),
            serial: AtomicU32::new(0),
        }
    }

    /// Spawn threads until the pool has the given size
    pub fn ensure_threads(&self, size: usize) -> Result<(), Error> {
        let mut handles = self.handles.lock().map_err(|_| Error::Lock)?;
        let mut spawner = self.spawner.lock().map_err(|_| Error::Lock)?;
        if handles.len() >= size {
            log::debug!(
                "already have {} threads, not creating new threads",
                handles.len()
            );
            return Ok(());
        }
        let to_create = size - handles.len();
        handles.reserve(to_create);
        log::info!("creating {size} threads");
        for i in 0..to_create {
            log::debug!("spawning processor thread {i}");
            let handle = spawner.spawn(i)?;
            handles.push(handle);
        }
        self.handles_len.store(handles.len(), Ordering::Release);
        log::info!("threads created successfully");
        Ok(())
    }

    /// Execute a job
    pub async fn execute<F, T>(&self, f: F) -> Result<T, Error>
    where
        F: FnOnce(&mut Cpu1) -> T + Send + 'static,
        T: Send + 'static,
    {
        // scheduling here is VERY broken. for some reason,
        // running on a single thread is the fastest, and even having
        // 2 threads slows everything down by A LOT
        // i thought the handles.lock() is having too much contention,
        // but that doesn't seem to be the case.
        //
        // In any case, don't look at the stuff below too much.
        // we will probably rewrite the entire execution model
        // once I think about how to make it work in both tokio/native
        // and WASM multithreading
        let (send, recv) = oneshot::channel();

        let i = {
            let serial = self
                .serial
                .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            loop {
                let handles_len = self.handles_len.load(Ordering::Acquire);
                if handles_len == 0 {
                    return Err(Error::EmptyPool);
                }
                let i = (serial as usize) % handles_len;
                {
                    let handles = self.handles.lock().map_err(|_| Error::Lock)?;
                    let Some((_, sender)) = handles.get(i) else {
                        continue;
                    };
                    log::debug!("executing job on processor thread {i}");
                    let _ = sender.send(Box::new(move |p| {
                        let t = f(p);
                        if send.send(t).is_err() {
                            log::error!("processor thread {i} failed to send result back");
                        }
                    }));
                };
                break i;
            }
        };

        log::debug!("waiting for processor thread {i}");
        let exec_result = { recv.await.map_err(|e| Error::RecvResult(e.to_string())) };
        let exec_error = match exec_result {
            Ok(t) => {
                log::debug!("received from processor thread {i}");
                return Ok(t);
            }
            Err(e) => e,
        };

        // if error happens, try to kill the thread and make a new one
        log::error!("failed to send job to processor thread {i}: {exec_error}",);
        log::info!("trying to spawn new processor thread");
        let (join, send) = {
            let mut handles = self.handles.lock().map_err(|_| Error::Lock)?;
            let mut spawner = self.spawner.lock().map_err(|_| Error::Lock)?;

            // create new processor thread
            let mut thread_holder = match spawner.spawn(i) {
                Ok(x) => x,
                Err(e) => {
                    log::error!("failed to spawn processor thread: {e}");
                    // leave the bad processor thread in place, and try next time..
                    return Err(e);
                }
            };
            log::info!("spawned new processor thread {i}");
            // let mut thread_holder = (thread_holder.0, Some(thread_holder.1));
            // remove the old thread
            std::mem::swap(&mut handles[i], &mut thread_holder);
            thread_holder
        };
        log::info!("stopping old processor thread {i}");
        drop(send);
        if let Err(e) = join.join() {
            log::error!("failed to join old processor thread {i}: {e}");
        }

        Err(exec_error)
    }
}
