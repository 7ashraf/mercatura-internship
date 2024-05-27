#[deny(clippy::all)]
use std::env;
use std::fmt::Result;

const API_KEY: &str = "https://vpic.nhtsa.dot.gov/api/vehicles/getallmanufacturers?format=json";
struct Manufacturer<'a> {
    name: Option<&'a str>,
    country: Option<&'a str>,
    common_name: Option<&'a str>,
}

trait Contains {
    fn contains(&self, needle:&str) -> bool;
}

impl<'a> Contains for Manufacturer<'a> {
    fn contains(&self, needle: &str) -> bool {
        self.name.unwrap_or("").contains(needle) ||
        self.country.unwrap_or("").contains(needle) ||
        self.common_name.unwrap_or("").contains(needle  )
    }
}

impl<'a> Manufacturer<'a> {
    fn description (&self) -> String {
        format!("{} ({}) - {}", self.name.unwrap_or("Unknown"), self.country.unwrap_or("Unknown"), self.common_name.unwrap_or("Unknown"))
    }

}

#[tokio::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <search term>", args[0]);
        return Ok(())
    }

    let keyword = &args[1];
    let client = reqwest::Client::new();
    let res = client
    .get(API_KEY)
    .send()
    .await?
    .json::serde_json::Value()
    .await?;
    println!("Hello, world!");
}
