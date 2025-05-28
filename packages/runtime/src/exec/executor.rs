
use std::sync::{atomic::AtomicU32, Mutex, RwLock};

use blueflame::processor::Cpu1;

use crate::exec::{Spawn, Join, JobSender, Error};

/// A non-work-stealing executor pool of emulated processors
///
/// When thread dies, it spawns new threads to replace the dead ones.
pub struct ExecutorImpl<S: Spawn> {
    /// Spawner to spawn new threads
    spawner: Mutex<S>,
    /// Handles to the spawned threads
    handles: RwLock<Vec<(S::Joiner, JobSender)>>,
    /// Serial number to round-robin the threads
    serial: AtomicU32,
}

impl<S: Spawn> ExecutorImpl<S> {
    /// Create a new executor. Note this does not spawn any threads yet
    pub fn new(spawner: S) -> Self {
        ExecutorImpl {
            spawner: Mutex::new(spawner),
            handles: RwLock::new(Vec::new()),
            serial: AtomicU32::new(0),
        }
    }

    /// Spawn threads until the pool has the given size
    pub fn ensure_threads(&self, size: usize) -> Result<(), Error> {
        {
            let handles = self.handles.read().map_err(|_| Error::Lock)?;
            if handles.len() >= size {
                log::info!("already have {} threads, not creating new threads", handles.len());
                return Ok(());
            }
        }
        {
            let mut handles = self.handles.write().map_err(|_| Error::Lock)?;
            if handles.len() >= size {
                log::info!("already have {} threads, not creating new threads", handles.len());
                return Ok(());
            }
            let to_create = size - handles.len();
            handles.reserve(to_create);
            log::info!("creating {} threads", size);
            for i in 0..to_create {
                log::info!("spawning processor thread {}", i);
                let mut spawner = self.spawner.lock().map_err(|_| Error::Lock)?;
                let handle = spawner.spawn(i)?;
                handles.push(handle);
            }
            log::info!("threads created successfully");
            Ok(())
        }
    }

    /// Execute a job
    pub async fn execute<F, T>(&self, f: F)  -> Result<T, Error>
    where 
        F: FnOnce(&mut Cpu1) -> T + Send + 'static,
        T: Send + 'static,
    {
        let (send, recv) = oneshot::channel();

        let i = {
            let handles = self.handles.read().map_err(|_| Error::Lock)?;
            if handles.is_empty() {
                return Err(Error::EmptyPool);
            }
            let serial = self.serial.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            let i = (serial as usize) % handles.len();

            log::debug!("executing job on processor thread {}", i);
            handles[i].1.send(Box::new(|p| {
                let t = f(p);
                let _ = send.send(t);
            })).map_err(|e| Error::SendJob(e.to_string()))?;

            i
        };

        let exec_result = recv.await.map_err(|e| Error::RecvResult(e.to_string()));
        let exec_error = match exec_result {
            Ok(t) => {
                log::debug!("processor thread {} finished job", i);
                return Ok(t);
            }
            Err(e) => e
        };

        // if error happens, try to kill the thread and make a new one
        log::error!("failed to send job to processor thread {}: {}", i, exec_error);
        log::info!("trying to spawn new processor thread");
        let (join, send) = {
            let mut handles = self.handles.write().map_err(|_| Error::Lock)?;
            let mut spawner = self.spawner.lock().map_err(|_| Error::Lock)?;

            // create new processor thread
            let mut thread_holder = match spawner.spawn(i) {
                Ok(x) => x,
                Err(e) => {
                    log::error!("failed to spawn processor thread: {}", e);
                    // leave the bad processor thread in place, and try next time..
                    return Err(e);
                }
            };
            log::info!("spawned new processor thread {}", i);
            // remove the old thread
            std::mem::swap(&mut handles[i], &mut thread_holder);
            thread_holder
        };
        log::info!("stopping old processor thread {}", i);
        drop(send);
        if let Err(e) = join.join() {
            log::error!("failed to join old processor thread {}: {}", i, e);
        }

        Err(exec_error)
    }
}
