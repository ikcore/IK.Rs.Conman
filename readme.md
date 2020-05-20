# Conman - Concurrency Manager

Conman is a perfomant thread safe, thread and work item management library.
Simple to use, just instantiate an instance of conman with the number of threads you wish to consume.
Impliment a ConmanItem trait on your struct and add it as an item. If there is a thread available it will be pulled instantly else it will be added to the queue and pulled by the next available thread.

There is no unsafe code used...

```toml
[dependencies]
conman = "0.1.6"
```

[Conman Repo](https://github.com/ikcore/IK.Rs.Conman "Conman Repository")

```Rust
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

    // new instance
    let mut cman = conman::Conman::new(8);

    for i in 0..100 {
        let item = Box::new(MyItem { id: i });

        // add an item
        cman.add_item(item);

        // add item at front of queue
        // cman.add_item_priority(item);
    }
    std::thread::sleep(Duration::from_millis(80));
    let remaining = cman.stop();
    println!("Stopped with {} items in queue", remaining);
}
```
