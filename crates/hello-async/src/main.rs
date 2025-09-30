use trpl::Html;
async fn page_title(url: &str) -> Option<String> {
    let response_text = trpl::get(url).await.text().await;
    Html::parse(&response_text)
        .select_first("title")
        .map(|title| title.inner_html())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    trpl::run(async {
        let url = &args[1];
        match page_title(url).await {
            Some(title) => {
                let title = title.trim();
                println!("Title: \"{title}\" for {url}");
            }
            None => println!("{url} had no title"),
        }
    })
}
