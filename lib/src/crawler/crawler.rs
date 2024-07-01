use url::Url;
use tokio::task;
use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;
use tokio::sync::mpsc::{channel, Sender};
use crate::crawler::fetch_urls;
use crate::config::calculate_optimal_threads;

type CrawlResult = Result<(), Box<dyn std::error::Error + Send + Sync + 'static>>;
type BoxFuture = std::pin::Pin<Box<dyn std::future::Future<Output = CrawlResult> + Send>>;

fn box_crawl(pages: Vec<Url>, current: u8, max: u8) -> BoxFuture {
    Box::pin(crawl(pages, current, max))
}

async fn crawl(pages: Vec<Url>, current: u8, max: u8) -> CrawlResult {
    println!("Current Depth: {}, Max Depth: {}", current, max);

    if current >= max {
        println!("Max Depth Reached");
        return Ok(());
    }

    let optimal_threads = calculate_optimal_threads();
    println!("Using {} threads for crawling.", optimal_threads);

    let to_visit = Arc::new(Mutex::new(pages));
    let visited = Arc::new(Mutex::new(HashSet::new()));
    let (tx, mut rx) = channel(optimal_threads);

    for _ in 0..optimal_threads {
        let to_visit = Arc::clone(&to_visit);
        let visited = Arc::clone(&visited);
        let tx = tx.clone();

        task::spawn(async move {
            crawl_worker(to_visit, visited, tx).await;
        });
    }

    drop(tx); // Close the sender to allow the receiver to complete

    let mut result_links = vec![];
    while let Some(urls) = rx.recv().await {
        result_links.extend(urls);
    }

    println!("Collected URLs: {:?}", result_links);

    Ok(())
}

async fn crawl_worker(
    to_visit: Arc<Mutex<Vec<Url>>>,
    visited: Arc<Mutex<HashSet<Url>>>,
    tx: Sender<Vec<Url>>,
) {
    while let Some(url) = {
        let mut to_visit = to_visit.lock().unwrap();
        to_visit.pop()
    } {
        if !visited.lock().unwrap().insert(url.clone()) {
            continue;
        }

        match fetch_urls(&url).await {
            Ok(links) => {
                tx.send(links).await.unwrap();
            }
            Err(e) => {
                println!("Error fetching page: {:?}", e);
            }
        }
    }
}

pub fn crawler_call(link: Url, current: u8, max: u8) {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        box_crawl(vec![link], current, max).await.unwrap();
    });
}
