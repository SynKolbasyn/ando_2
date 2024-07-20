mod net;
mod cache;
mod parser;
mod cli;
mod anime;
mod settings;

use crate::cli::CLI;


#[tokio::main]
async fn main() {
    let mut cli: CLI = CLI::default();
    match cli.start().await {
        Ok(_) => (),
        Err(e) => eprintln!("CRITICAL ERROR: {e}"),
    }
    // loop {
    //     match cli.start() {
    //         Ok(_) => break,
    //         Err(e) => eprintln!("CRITICAL ERROR: {e}"),
    //     }
    // }
}
