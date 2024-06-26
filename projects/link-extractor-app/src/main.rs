use error_chain::error_chain;
use select::document::Document;
use select::predicate::Name;

error_chain! {
    foreign_links {
        IoError(std::io::Error);
        ReqError(reqwest::Error);
    }
}

#[tokio::main]
async fn main() -> Result<()>{
    let body = reqwest::get("https://www.rust-lang.org").await?.text().await?;
    let document = Document::from(body.as_str())
    .find(Name("a"))
    .filter_map(|n| n.attr("href"))
    .for_each(|x| println!("{}", x));
    println!("Hello, world!");
    Ok(())
}
