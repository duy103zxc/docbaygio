use crate::config::run;
use crate::config::Config;
use std::env;

mod config;
mod epub;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    println!("Docbaygio đang tải nội dung cho bạn, lỗi có thể xảy ra trong quá trình tải xuống. Bạn cần nhập đúng lệnh và có kết nối Internet để thực hiện nha <3!");
    let args = env::args();
    let new_args = Config::new(args)?;
    run(new_args)?;
    Ok(())
}
