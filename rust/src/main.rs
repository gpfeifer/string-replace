
use std::time::Instant;
use std::fs::{read_to_string};

// Run with: cargo run --release 
fn main() {
    let text = read_to_string("../string.json").unwrap();
    let parse = Instant::now();
    let _result = text.replace("http://localhost:35261/product", "http://example.com/product");
    println!("Replace {} nanos", parse.elapsed().as_nanos());
}

