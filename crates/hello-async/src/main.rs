use std::{
    pin::{Pin, pin},
    thread,
    time::Duration,
};
use trpl::{Either, Html, ReceiverStream, Stream, StreamExt};

#[allow(dead_code)]
async fn page_title(url: &str) -> (&str, Option<String>) {
    let response_text = trpl::get(url).await.text().await;
    let title = Html::parse(&response_text)
        .select_first("title")
        .map(|title| title.inner_html());
    (url, title)
}

#[allow(dead_code)]
fn webscraper() {
    let args: Vec<String> = std::env::args().collect();

    trpl::run(async {
        let title_fut_1 = page_title(&args[1]);
        let title_fut_2 = page_title(&args[2]);

        let (url, maybe_title) = match trpl::race(title_fut_1, title_fut_2).await {
            Either::Left(left) => left,
            Either::Right(right) => right,
        };

        println!("{url} returned first");

        match maybe_title {
            Some(title) => {
                let title = title.trim();
                println!("Page Title: {title}");
            }
            None => println!("Had no title"),
        }
    });
}

#[allow(dead_code)]
fn spawn_tasks() {
    trpl::run(async {
        let h1 = trpl::spawn_task(async {
            for i in 1..10 {
                println!("Hi {i} from task 1");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });

        for i in 1..5 {
            println!("Hi {i} from task 2");
            trpl::sleep(Duration::from_millis(500)).await;
        }

        h1.await.unwrap();
    });
}

#[allow(dead_code)]
fn task_join() {
    trpl::run(async {
        let fut1 = async {
            for i in 1..10 {
                println!("{i} from task 1");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let fut2 = async {
            for i in 1..5 {
                println!("{i} from task 2");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        trpl::join(fut1, fut2).await;
    });
}

#[allow(dead_code)]
fn task_messages() {
    let f1 = async {
        let (tx, mut rx) = trpl::channel();

        let val = String::from("°w°");
        tx.send(val).unwrap();

        let received = rx.recv().await.unwrap();
        println!("Received: {received}");
    };

    trpl::block_on(f1);
}

#[allow(dead_code)]
fn task_moar_messages() {
    trpl::run(async {
        let (tx, mut rx) = trpl::channel();

        let tx1 = tx.clone();
        let tx_fut = pin!(async move {
            let vals = vec![
                String::from("These"),
                String::from("are"),
                String::from("some"),
                String::from("messages"),
                String::from("END"),
            ];

            for val in vals {
                tx1.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });

        let rx_fut = pin!(async {
            while let Some(value) = rx.recv().await {
                println!("Received: {value}");
            }
        });

        let tx_fut2 = pin!(async move {
            let vals = vec![
                String::from("Some"),
                String::from("more"),
                String::from("messages"),
                String::from("UwU"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(1500)).await;
            }
        });

        // trpl::join!(tx_fut, rx_fut, tx_fut2);
        let futures: Vec<Pin<&mut dyn Future<Output = ()>>> = vec![tx_fut, tx_fut2, rx_fut];
        trpl::join_all(futures).await;
    });
}

fn slow(name: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("'{name}' ran for {ms}ms");
}

#[allow(dead_code)]
fn yield_async() {
    trpl::block_on(async {
        let a = async {
            let tasks = vec![30, 20, 10];
            println!("'a' started.");

            for task in tasks {
                slow("a", task);
                trpl::yield_now().await;
            }

            println!("'a' finished.");
        };

        let b = async {
            let tasks = vec![75, 10, 15, 350];
            println!("'b' started.");

            for task in tasks {
                slow("b", task);
                trpl::yield_now().await;
            }

            println!("'b' finished.");
        };
        trpl::select(a, b).await;
    });
}

async fn timeout<F: Future>(future_to_try: F, max_time: Duration) -> Result<F::Output, Duration> {
    match trpl::select(future_to_try, trpl::sleep(max_time)).await {
        Either::Left(output) => Ok(output),
        Either::Right(_) => Err(max_time),
    }
}

#[allow(dead_code)]
fn custom_timeout() {
    trpl::block_on(async {
        let slow = async {
            trpl::sleep(Duration::from_secs(5)).await;
            "I finished!"
        };

        match timeout(slow, Duration::from_secs(2)).await {
            Ok(message) => println!("Succeeded with: '{message}'"),
            Err(duration) => println!("Failed after {}s", duration.as_secs()),
        }
    })
}

#[allow(dead_code)]
fn stream_test() {
    trpl::block_on(async {
        let a = vec![1, 2, 3, 4, 5, 9, 10, 12];
        let iter = a.iter().map(|val| val * 2);
        let mut stream = trpl::stream_from_iter(iter);

        while let Some(val) = stream.next().await {
            println!("{val}");
        }
    });
}

fn get_messages() -> impl Stream<Item = String> {
    let (tx, rx) = trpl::channel();

    trpl::spawn_task(async move {
        let messages = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];
        for (index, message) in messages.into_iter().enumerate() {
            let time_to_sleep = if index % 2 == 0 { 100 } else { 300 };
            trpl::sleep(Duration::from_millis(time_to_sleep)).await;

            if let Err(e) = tx.send(format!("Message: '{message}'")) {
                eprintln!("Cannot send message '{message} : {e}'");
                break;
            }
        }
    });

    ReceiverStream::new(rx)
}

fn get_intervals() -> impl Stream<Item = u32> {
    let (tx, rx) = trpl::channel();

    trpl::spawn_task(async move {
        let mut count = 0;
        loop {
            trpl::sleep(Duration::from_millis(1)).await;
            count += 1;
            if let Err(e) = tx.send(count) {
                eprintln!("Unable to send Interval {count} : {e}");
            }
        }
    });

    ReceiverStream::new(rx)
}

#[allow(dead_code)]
fn receiver_stream() {
    trpl::block_on(async {
        let messages = get_messages().timeout(Duration::from_millis(200));
        let intervals = get_intervals()
            .map(|count| format!("Interval: {count}"))
            .throttle(Duration::from_millis(100))
            .timeout(Duration::from_secs(10));
        let merged = messages.merge(intervals).take(20);
        let mut stream = pin!(merged);

        while let Some(result) = stream.next().await {
            match result {
                Ok(message) => println!("{message}"),
                Err(reason) => eprintln!("Problem: {reason:?}"),
            }
        }
    })
}

fn main() {
    receiver_stream();
}
