pub mod crawler;
pub mod utils;
mod scrapper;

pub use crawler::{crawl, box_crawl, get_links, CrawlResult, BoxFuture};
pub use utils::link_queue;
