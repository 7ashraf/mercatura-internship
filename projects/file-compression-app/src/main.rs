extern crate flate2;

use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs::File;
use std::env::args;
use std::io::copy;
use std::io::BufReader;
use std::time::Instant;



fn main() {
    if args().len() != 3 {
        eprintln!("Usage: {} <input> <output>", args().nth(0).unwrap());
        std::process::exit(1);
    }
    let mut input = BufReader::new(File::open(args().nth(1).unwrap()).unwrap());
    let output = File::create(args().nth(2).unwrap()).unwrap();
    let mut encoder = GzEncoder::new(output, Compression::default());
    let start = Instant::now();
    copy(&mut input, &mut encoder).unwrap();
    let output = encoder.finish().unwrap();
    println!("Elapsed time: {:.2?}", start.elapsed());
    println!("Original size: {} bytes", input.get_ref().metadata().unwrap().len());
    println!("Compressed size: {} bytes", output.metadata().unwrap().len());
    
    println!("Hello, world!");
}
