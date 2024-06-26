use url::Url;
use surf;

use crate::crawler::get::get_links;

pub async fn fetch_page(url: &Url) -> Result<Vec<Url>, Box<dyn std::error::Error + Send + Sync>> {
    println!("Fetching: {}", url);
    let mut res = surf::get(url).await?;
    let body = res.body_string().await?;
    let links = get_links(url, body);
    Ok(links)
}
