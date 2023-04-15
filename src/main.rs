use std::env;
use serde_json::Value;

fn main() {
    let data: Value = serde_json::from_str(
        include_str!("data/names.json")
    ).unwrap();

    let names: Vec<String> = (data["names"])
        .as_array().unwrap().to_vec()
        .iter()
        .map(|s| s.as_str().unwrap().to_string())
        .collect();

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 || !names.contains(&args[1]) {
        eprintln!("Usage: {} <option>", args[0]);
        // exit here with status code 1
        std::process::exit(1);
    }
    println!("Hello, {}!", args[1]);
}
