use async_std::task;
use url::Url;
use rust_scrapper::box_crawl;

fn main() {
    task::block_on(async {
        box_crawl(vec![Url::parse("https://www.insidr.ai/ai-tools/").unwrap()], 1, 2).await.unwrap();
    });
}
