use rust_scrapper::exec_crawler;

fn main() {
    exec_crawler("https://www.rust-lang.org/".parse().unwrap(), 1, 2);
}
