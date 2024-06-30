use rust_scrapper::crawler_call;

fn main() {
    let url= "https://www.rust-lang.org/"
        .parse().unwrap();

    crawler_call(url, 1, 2);
}

