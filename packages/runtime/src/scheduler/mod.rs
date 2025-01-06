use blueflame::processor::Processor;

pub trait Scheduler: Clone {
    /// Future type for this scheduler
    type Future<T>: std::future::Future<Output = T> + Send + 'static;

    /// Schedule a task a on a processor
    fn run_on_core<T, F: FnOnce(&mut Processor) -> T>(&self, f: F) -> Self::Future<T>;
}
