use url::{ Url, ParseError };
use html5ever::tokenizer::{ BufferQueue, Tokenizer, TokenizerOpts };

use crate::crawler::utils::LinkQueue;

pub fn get_adjacent_links(url: &Url, page: String) -> Vec<Url> {
    let mut domain_url = url.clone();
    domain_url.set_path("");
    domain_url.set_query(None);

    let mut queue = LinkQueue::default();
    let mut tokenizer = Tokenizer::new(&mut queue, TokenizerOpts::default());
    let mut buffer = BufferQueue::default();
    buffer.push_back(page.into());
    let _ = tokenizer.feed(&mut buffer);

    queue.links.iter()
        .map(|link| match Url::parse(link) {
            Err(ParseError::RelativeUrlWithoutBase) => domain_url.join(link).unwrap(),
            Err(_) => panic!("Relative URL {}", link),
            Ok(url) => url,
        }).collect()
}
