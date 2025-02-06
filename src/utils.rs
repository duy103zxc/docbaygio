use std::{fs::{self, File, OpenOptions}, path::Path};

use ureq::http::Response;

pub fn fetching_content_from_url(url: &str) -> Result<Response<ureq::Body>, ureq::Error>  {
    ureq::get(url)
        .header("User-Agent", "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36")
        .call()
}

pub fn fetching_page_title(url: &str) -> String {
    String::from("")
}

pub fn generate_writable_file(file_name: &str, file_format: &str) -> Result<File, std::io::Error> {
    OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(format!("{}.{}", file_name, file_format))
}

pub fn gen_urls(text_file: &str) -> anyhow::Result<Vec<String>> {
    let links: Vec<String> = fs::read_to_string(text_file)?.lines().map(|line| String::from(line)).collect();
    Ok(links)
}

