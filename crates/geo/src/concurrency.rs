use std::thread;
use std::time::Duration;

pub fn thread_test() {
    let thr = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from spawned thread");
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..5 {
        println!("hi number {i} from main thread");
        thread::sleep(Duration::from_millis(1));
    }
    thr.join().unwrap(); // with this the main thread waits for the child thread to finish
}

pub fn capture_thread_test() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Vector form thread: {v:?}");
    });

    handle.join().unwrap();
}
