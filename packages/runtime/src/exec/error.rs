/// Error in executor
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("failed to create spawner")]
    CreateSpawner,
    #[error("failed to create thread: {0}")]
    CreateThread(String),
    #[error("no threads left in the pool")]
    EmptyPool,
    #[error("failed to send job to processor thread: {0}")]
    SendJob(String),
    #[error("failed to recv result from processor thread, thread is lost: {0}")]
    RecvResult(String),
    #[error("failed to join processor thread: {0}")]
    Join(String),
    #[error("failed to acquire lock")]
    Lock,
}
