use std::{env, process};
use args::Config;

mod utils;
mod args;
fn main() -> anyhow::Result<()> {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    utils::download_the_post(&config.url, &config.filename)?;
    println!("Downloading the post from the Internet...");

    Ok(())
}


