
use std::sync::{Arc, Mutex};
mod item;
pub use item::ConmanItem;
mod manager;
pub use manager::ConmanManager;
mod processor;
pub use processor::ConmanWorkerProcessor;
mod worker;
pub use worker::ConmanWorker;

pub struct Conman {
    manager: Arc<Mutex<ConmanManager>>,
    workers: Vec<ConmanWorker>
}
impl Conman {
    pub fn add_item (&self, item: Box<dyn ConmanItem + Send>) {
        let mut mgr = self.manager.lock().unwrap();
        mgr.add_item(item);
        self.signal_workers(); 
    }
    fn signal_workers(&self) {
        for worker in &self.workers {
            if worker.signal() {
                break;
            }
        }
    }
    pub fn new(num_threads: usize) -> Conman {
        let mut workers = Vec::with_capacity(num_threads);
        let manager = Arc::new(Mutex::new(ConmanManager::new()));
        for _i in 0..num_threads {
            workers.push(ConmanWorker::new(manager.clone()));
        }
        Conman { workers, manager }
    }
    #[allow(dead_code)]
    pub fn stop(&mut self) -> usize {
        for worker in &self.workers {
            worker.stop();
        }
        let num_workers = self.workers.len();
        for _i in 0..num_workers {
            let _res = self.workers.pop();
        }
        let mgr = self.manager.lock().unwrap();
        return mgr.items.len();
    }
    #[allow(dead_code)]
    pub fn get_num_items(&self) -> usize {
        let mgr = self.manager.lock().unwrap();
        return mgr.items.len();
    }
}