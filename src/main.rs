mod net;
mod cache;
mod parser;
mod cli;
mod anime;
mod settings;


use crate::cli::CLI;


#[tokio::main]
async fn main() {
    loop {
        let mut cli: CLI = CLI::default();
        match cli.start().await {
            Ok(_) => break,
            Err(e) => {
                eprintln!("CRITICAL ERROR: {e}");
                eprintln!("Restarting...");
            },
        }
    }
}
