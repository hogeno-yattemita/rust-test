
use std::{thread, time::Duration};

fn main() {
    let handle = thread::spawn(|| {
        for i in 0..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    // The thread is still running
    handle.join().unwrap();

    for i in 0..10 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

}
