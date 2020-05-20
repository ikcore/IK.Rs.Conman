#Conman - Concurrency Manager

Conman is a perfomant thread safe, thread and work item management library.
Simple to use, just instantiate an instance of conman with the number of threads you wish to consume.
Impliment a ConmanItem trait on your struct and add it as an item. If there is a thread available it will be pulled instantly else it will be added to the queue and pulled by the next available thread.

There is no unsafe code used...

[link](https://github.com/ikcore/IK.Rs.Conman "Conman Repository")

```rust,skt-template
pub extern crate conman;
pub use conman::*;

struct MyItem {
    id: u32
}
impl conman::ConmanItem for MyItem {
    fn execute (&self) {
        println!("{}", self.id);
    }
}
fn main() {

    // I like to use num_cpus to get the number of logical cores available
    // let mut cman = conman::Conman::new(num_cpus::get());

    let mut cman = conman::Conman::new(8);

    for i in 0..100 {
        let item = Box::new(MyItem { id: i });
        cman.add_item(item);
    }
    std::thread::sleep(Duration::from_millis(80));
    let remaining = cman.stop();
    println!("Stopped with {} items in queue", remaining);    
}
```