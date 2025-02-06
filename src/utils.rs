use std::{error::Error, fs::{File, OpenOptions}, io::Write};

use nanohtml2text::html2text;
use sanitize_html::{errors::SanitizeError, rules::predefined::BASIC, sanitize_str};
use ureq::http::Response;

pub fn fetching_content_from_url(url: &str) -> Result<Response<ureq::Body>, ureq::Error>  {
    ureq::get(url)
        .header("User-Agent", "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36")
        .call()
}

/*
!!!
How to deal with Result<> with multiple types of errors in one function 
*/
pub fn download_the_post(url: &str, file_name: String) -> Result<String, &impl Error> {
    sanitize_str(&BASIC, &fetching_content_from_url(url)?
    .body_mut()
    .read_to_string()?)
}


pub fn generate_writable_file(file_name: &str, file_format: &str) -> Result<File, std::io::Error> {
    OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(format!("{}.{}", file_name, file_format))
}

