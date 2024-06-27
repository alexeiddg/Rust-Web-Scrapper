use url::Url;
use tokio::task;
use std::collections::HashSet;
use tokio::runtime::Runtime;
use std::sync::{ Arc, Mutex };
use tokio::sync::mpsc::{ channel, Sender };

use crate::crawler::fetch::fetch_page;

const THREADS: usize = 20;

pub type CrawlResult = Result<(), Box<dyn std::error::Error + Send + Sync + 'static>>;
pub type BoxFuture = std::pin::Pin<Box<dyn std::future::Future<Output = CrawlResult> + Send>>;

pub fn box_crawl(pages: Vec<Url>, current: u8, max: u8) -> BoxFuture {
    Box::pin(crawl(pages, current, max))
}

pub async fn crawl(pages: Vec<Url>, current: u8, max: u8) -> CrawlResult {
    println!("Current Depth: {}, Max Depth: {}", current, max);

    if current >= max {
        println!("Max Depth Reached");
        return Ok(());
    }

    let to_visit = Arc::new(Mutex::new(pages));
    let visited = Arc::new(Mutex::new(HashSet::new()));
    let (tx, mut rx) = channel(THREADS);

    for _ in 0..THREADS {
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

        match fetch_page(&url).await {
            Ok(links) => {
                tx.send(links).await.unwrap();
            }
            Err(e) => {
                println!("Error fetching page: {:?}", e);
            }
        }
    }
}

pub fn exec_crawler(link: Url, current: u8, max: u8) {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        box_crawl(vec![link], current, max).await.unwrap();
    });
}
