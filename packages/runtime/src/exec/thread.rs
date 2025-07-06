use std::sync::mpsc;

use blueflame::processor::Cpu1;

use crate::exec::{Job, JobSender};

/// Struct representing resources held by one thread
///
/// The thread can be made with [`make_thread`], which will return
/// the thread object and a sender to send jobs to the thread.
pub struct Thread {
    slot: usize,
    recv: mpsc::Receiver<Job>,
    cpu: Cpu1,
}

impl Thread {
    /// Execute the main loop of the processor thread, which waits for jobs
    /// to be sent from the main thread and executes them
    pub fn main_loop(mut self) {
        log::info!("processor thread {} started", self.slot);
        loop {
            match self.recv.recv() {
                Ok(f) => {
                    log::debug!("processor thread {} got job", self.slot);
                    
                    f(&mut self.cpu);
                    
                    // since mut references are not UnwindSafe, we must
                    // take it out, give it to the closure, then back
                    // let cpu = std::mem::take(&mut self.cpu);
                    // let result = std::panic::catch_unwind(move || {
                    //     f(cpu)
                    // });
                    // match result {
                    //     Ok(cpu) => self.cpu = cpu,
                    //     Err(e) => {
                    //         let msg = if let Some(e) = e.downcast_ref::<&str>() {
                    //             *e
                    //         } else if let Some(e) = e.downcast_ref::<String>() {
                    //             e.as_str()
                    //         } else {
                    //             "unknown panic payload"
                    //         };
                    //         log::error!("executor thread panicked: {msg}");
                    //         panic!("executor thread panicked: {msg}");
                    //     }
                    // }

                }
                Err(e) => {
                    log::debug!(
                        "processor thread {} failed to receive job, sender must have been dropped: {}",
                        self.slot,
                        e
                    );
                    break;
                }
            }
        }
        log::info!("processor thread {} exiting", self.slot);
    }
}

pub fn make_thread(slot: usize, cpu: Cpu1) -> (Thread, JobSender) {
    let (send, recv) = mpsc::channel();
    let thread = Thread { slot, recv, cpu };
    (thread, send)
}
