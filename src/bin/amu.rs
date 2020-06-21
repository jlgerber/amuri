use amuri::parse::uri::parse_uri;
use amuri::query::client::Client;
use std::env;
use amuri::traits::Retriever;
use amuri::assetmodel::AssetModelOwned;

pub fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: amu <uri>");
        std::process::exit(0);
    }
    let results = parse_uri(&args[1]);
    match results {
        Ok(asset_model) => {
            let client = match Client::from_env() {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(0);
                }
            };
            let query_results = client.get(&AssetModelOwned::from(asset_model));
            match query_results {
                Ok(file) => println!("{}", file),
                Err(e) => println!("\nERROR:\n{}\n", e),
            }
        }
        Err(e) => println!("\nERROR:\n{}\n", e),
    }
}
