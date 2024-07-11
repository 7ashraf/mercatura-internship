#[deny(clippy::all)]
use std::env;
//use std::fmt::Result;

const API_KEY: &str = "https://vpic.nhtsa.dot.gov/api/vehicles/getallmanufacturers?format=json";
struct Manufacturer<'a> {
    name: Option<&'a str>,
    country: Option<&'a str>,
    common_name: Option<&'a str>,
}

trait Contains {
    fn contains(&self, needle: &str) -> bool;
}

impl<'a> Contains for Manufacturer<'a> {
    fn contains(&self, needle: &str) -> bool {
        self.name.unwrap_or("").contains(needle)
            || self.country.unwrap_or("").contains(needle)
            || self.common_name.unwrap_or("").contains(needle)
    }
}

impl<'a> Manufacturer<'a> {
    fn description(&self) -> String {
        format!(
            "{} ({}) - {}",
            self.name.unwrap_or("Unknown"),
            self.country.unwrap_or("Unknown"),
            self.common_name.unwrap_or("Unknown")
        )
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <search term>", args[0]);
        return Ok(());
    }

    let keyword = &args[1];
    let client = reqwest::Client::new();
    let res = client
        .get(API_KEY)
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;
    println!("Hello, world!");

    let manufacturer_json = res
        .as_object()
        .unwrap()
        .iter()
        .find(|(key, _)| key == &"Results")
        .unwrap()
        .1
        .as_array()
        .unwrap()
        .iter();

    let manufacturers = manufacturer_json.map(|m| Manufacturer {
        name: m.get("Mfr_Name").and_then(|n| n.as_str()),
        country: m.get("Country").and_then(|n| n.as_str()),
        common_name: m.get("CommonName").and_then(|n| n.as_str()),
    });

    let found_manufacturers = manufacturers
        .filter(|m| m.contains(keyword))
        .collect::<Vec<Manufacturer>>();

    if found_manufacturers.is_empty() {
        //println!("No manufacturers found for '{}'", keyword);
        Err("No manufacturers found for".into())
    } else {
        println!(
            "{} manufacturers found for '{}'",
            found_manufacturers.len(),
            keyword
        );
        for (index, man) in found_manufacturers.iter().enumerate() {
            println!("#{}", index + 1);
            println!("{}", man.description())
        }
        Ok(())
    }
}
