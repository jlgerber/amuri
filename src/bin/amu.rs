use amuri::errors::AmuriError;
use amuri::parse::uri::parse_uri;
use std::env;

pub fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: amu <uri>");
        std::process::exit(0);
    }
    let results = parse_uri(&args[1]);
    match results {
        Ok(v) => println!("{:#?}", v),
        Err(e) => println!("\nERROR:\n{}\n", e),
    }
}
