use std::sync::{Arc, Mutex, mpsc};
#[allow(unused)]
use std::{thread, time::Duration};

pub fn thread_test() {
    let thr = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from spawned thread");
            // thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..5 {
        println!("hi number {i} from main thread");
        // thread::sleep(Duration::from_millis(1));
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

pub fn thread_channels() {
    let (tx, rx) = mpsc::channel();

    let handle = thread::spawn(move || {
        let val = String::from("OwO");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {received}");

    handle.join().unwrap();
}

pub fn thread_channels2() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("OwO"),
            String::from("What's this?"),
            String::from("UwU"),
            String::from("°w°"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            // thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {received}");
    }
}

pub fn multi_sender() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx1.send(val).unwrap();
            // thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("hai ^^"),
            String::from("messages"),
            String::from("for"),
            String::from("yous"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            // thread::sleep(Duration::from_secs(1));
        }
    });

    for msg in rx {
        println!("Got: {msg}");
    }
}

pub fn first_mutex() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {m:?}");
}

pub fn multi_thread_mutex() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Results: {}", *counter.lock().unwrap());
}

pub fn deadlock_test() {
    let counter = Arc::new(Mutex::new(0));
    let value = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    let c1 = Arc::clone(&counter);
    let v1 = Arc::clone(&value);
    let h1 = thread::spawn(move || {
        if let Ok(mut x) = c1.lock() {
            println!("Aquired c");
            thread::sleep(Duration::from_secs(3));
            *x += 1;
            println!("Trying to get v");
            let mut a = v1.lock().unwrap();
            *a += 1;
        }
    });
    handles.push(h1);

    let c2 = Arc::clone(&counter);
    let v2 = Arc::clone(&value);
    let h2 = thread::spawn(move || {
        if let Ok(mut x) = v2.lock() {
            println!("Aquired v");
            *x += 1;
            println!("Trying to get c");
            let mut a = c2.lock().unwrap();
            *a += 1;
        }
    });
    handles.push(h2);

    for handle in handles {
        handle.join().unwrap();
    }
}
