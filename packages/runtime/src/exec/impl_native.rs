use std::thread::JoinHandle;

use crate::exec::{self, Error, Job, JobSender, Join, Spawn};

pub struct Spawner;
impl Spawn for Spawner {
    type Joiner = JoinHandle<()>;

    fn spawn(&mut self, slot: usize) -> Result<(Self::Joiner, JobSender), Error> {
        let (thread, handle) = exec::make_thread(slot, Default::default());
        // block until Processor is fixed to be Send
        todo!();
        // let join_handle = std::thread::spawn(move || {
        //     thread.main_loop();
        // });
        // Ok((JoinHandle(join_handle), handle))
    }
}

impl Join for JoinHandle<()> {
    fn join(self) -> Result<(), Error> {
        self.join()
            .map_err(|_| Error::Join("failed to join thread".to_string()))
    }
}
