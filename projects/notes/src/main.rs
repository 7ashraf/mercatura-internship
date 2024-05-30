#![deny(clippy::all)]
use std::env;
use std::fs::OpenOptions;
use std::io::prelude::*;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: notes {}", args[0]);
        std::process::exit(1);
    }
    let note = args[1].clone();
    let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .create(true)
        .open("notes.txt")
        .unwrap();

    file.write_all(now.to_string().as_bytes())?;
    file.write_all(b" ")?;
    file.write_all(note.as_bytes())?;
    file.write_all(b"\n")?;

    println!("Hello, world!");
    Ok(())
}
