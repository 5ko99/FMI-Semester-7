use std::{thread, time};
fn main() {
    let handle = thread::spawn(|| println!("hi from spawned thread"));
    thread::sleep(time::Duration::from_millis(1));
    println!("hi from main thread");
    let _ = handle.join();
}
