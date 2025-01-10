use std::{fs::{File, OpenOptions}, io::Write};

use html2md::parse_html;
use html_purifier::{purifier, Settings};
use ureq::{Agent, Response};

pub fn fetching_content_from_url(url: &str, refer_site: &str) -> Result<Response, ureq::Error> {
    let user_agent = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36";
    
    let ureq_agent: Agent = ureq::AgentBuilder::new()
    .build();

    let body = ureq_agent.get(url)
        .set("User-Agent", user_agent)
        .set("Referer", refer_site)
        .call();
    body
}

pub fn clean_raw_response(html_input: &str) -> String {
    let settings = Settings {
        ..Settings::default()
    };
    let output = purifier(html_input, settings);
    return output;
}


pub fn download_the_post(url: &str, file_name: &str) -> anyhow::Result<()> {
    // 1. Downloading the post from the internet
    let raw_content = fetching_content_from_url(url, url)?.into_string()?;
    // 2. Clean the HTML
    let cleaned_content = clean_raw_response(&raw_content);
    // 3. Convert it into Markdown
    let output = parse_html(&cleaned_content);
    // 4. Save it into a new file (? name)
    let mut new_file = generate_writable_file(file_name, "md")?;
    new_file.write_all(output.as_bytes())?;

    Ok(())
}


pub fn generate_writable_file(file_name: &str, file_format: &str) -> Result<File, std::io::Error> {
    OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(format!("{}.{}", file_name, file_format))
}

