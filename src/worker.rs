use std::sync::atomic::{ Ordering };
use std::{thread};
use std::sync::{Arc, Mutex};
use super::{ ConmanManager, ConmanWorkerProcessor };

pub struct ConmanWorker {
    pub processor: Arc<ConmanWorkerProcessor>,
    pub thread_handle: Option<std::thread::JoinHandle<()>>
}
impl ConmanWorker {
    pub fn new(manager: Arc<Mutex<ConmanManager>> ) -> ConmanWorker {
        let processor = Arc::new(ConmanWorkerProcessor::new(manager));
        let inner_processor = processor.clone();
        let thread_handle = Option::Some(thread::Builder::new().spawn(move || {
            inner_processor.execute();
        }).unwrap());
        ConmanWorker { thread_handle, processor }
    }
    pub fn signal (&self) -> bool {
        if self.processor.is_busy.load(Ordering::Relaxed) {
            return false;
        }
        let (_lock, cvar) = &*self.processor.sync_object;
        cvar.notify_one();
        return true;
    }
    pub fn stop(&self) {
        self.processor.running.store(false, Ordering::Relaxed);
        let (_lock, cvar) = &*self.processor.sync_object;
        cvar.notify_one();
    }
    pub fn join(&mut self) {
        let ohandle = self.thread_handle.take();
        if ohandle.is_some() {
            ohandle.unwrap().join().expect("");
        }
    }
}
impl Drop for ConmanWorker {
    fn drop(&mut self) {
        self.stop();
        self.join();
    }
}