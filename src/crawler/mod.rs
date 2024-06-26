use async_std::task;
use surf;
use url::{ParseError, Url};
use html5ever::tokenizer::{BufferQueue, Tokenizer, TokenizerOpts};
use crate::utils::link_queue::LinkQueue;

pub type CrawlResult = Result<(), Box<dyn std::error::Error + Send + Sync + 'static>>;
pub type BoxFuture = std::pin::Pin<Box<dyn std::future::Future<Output = CrawlResult> + Send>>;

pub fn get_links(url: &Url, page: String) -> Vec<Url> {
    let mut domain_url = url.clone();
    domain_url.set_path("");
    domain_url.set_query(None);

    let mut queue = LinkQueue::default();
    let mut tokenizer = Tokenizer::new(&mut queue, TokenizerOpts::default());
    let mut buffer = BufferQueue::default(); // Initialize BufferQueue correctly
    buffer.push_back(page.into());
    let _ = tokenizer.feed(&mut buffer);

    queue.links.iter()
        .map(|link| match Url::parse(link) {
            Err(ParseError::RelativeUrlWithoutBase) => domain_url.join(link).unwrap(),
            Err(_) => panic!("Relative URL {}", link),
            Ok(url) => url,
        }).collect()
}

pub fn box_crawl(pages: Vec<Url>, current: u8, max: u8) -> BoxFuture {
    Box::pin(crawl(pages, current, max))
}

pub async fn crawl(pages: Vec<Url>, current: u8, max: u8) -> CrawlResult {
    println!("Current Depth: {}, Max Depth: {}", current, max);

    if current >= max {
        println!("Max Depth Reached");
        return Ok(());
    }

    let mut tasks = vec![];
    println!("Crawling: {:?}", pages);

    for url in pages {
        let task = task::spawn(async move {
            println!("Getting: {}", url);

            let mut res = surf::get(&url).await?;
            let body = res.body_string().await?;
            let links = get_links(&url, body);

            println!("Following: {:?}", links);
            box_crawl(links, current + 1, max).await
        });
        tasks.push(task);
    }

    for task in tasks.into_iter() {
        task.await?;
    }

    Ok(())
}
