use std::sync::{Arc, Mutex, Condvar};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use super::{ ConmanManager, ConmanItem };

pub struct ConmanWorkerProcessor {
    pub running: AtomicBool,
    pub is_busy: AtomicBool,
    pub manager: Arc<Mutex<ConmanManager>>,
    pub sync_object: Arc<(Mutex<bool>, Condvar)>
}
impl ConmanWorkerProcessor {
    pub fn new (manager: Arc<Mutex<ConmanManager>> ) -> ConmanWorkerProcessor {
        let sync_object_pair = Arc::new((Mutex::new(false), Condvar::new()));
        let sync_object = sync_object_pair.clone();
        ConmanWorkerProcessor {
            running: AtomicBool::new(true),
            is_busy: AtomicBool::new(false),
            sync_object,
            manager
        }
    }
    pub fn execute(&self) {
        while self.running.load(Ordering::Relaxed) {
            let mut processed_item : bool = false;
            let mut _item : Option<Box<dyn ConmanItem + Send>> = None;
            {
                let mut mgr = self.manager.lock().unwrap();
                _item = mgr.get_next_item();
            }
            match _item {
                Some(x) => {
                    self.is_busy.store(true, Ordering::Relaxed);
                    x.execute();
                    processed_item = true;
                    self.is_busy.store(false, Ordering::Relaxed);
                },
                None => {}
            }
            if !processed_item {
                let (lock, cvar) = &*self.sync_object;
                let mut _started = lock.lock().unwrap();
                'running: loop {
                    let _result = cvar.wait_timeout(_started, Duration::from_millis(500)).unwrap();
                    break 'running
                }
            }
        }
    }
}