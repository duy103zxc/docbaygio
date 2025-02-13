use crate::config::run;
use crate::config::Config;
use std::env;

mod config;
mod epub;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    println!("Downloading post(s) from the Internet");
    let args = env::args();
    let new_args = Config::new(args)?;
    run(new_args)?;
    Ok(())
}
