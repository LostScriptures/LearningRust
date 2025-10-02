use std::time::Duration;
use trpl::{Either, Html};

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

fn main() {
    task_join();
}
