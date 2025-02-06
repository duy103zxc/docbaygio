use std::{error::Error, fs::{self, File, OpenOptions}, io::Write, path::Path};
use epub_builder::{EpubBuilder, EpubContent, ReferenceType, ZipLibrary};
use ureq::http::Response;

pub fn fetching_content_from_url(url: &str) -> Result<Response<ureq::Body>, ureq::Error>  {
    ureq::get(url)
        .header("User-Agent", "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36")
        .call()
}

pub fn fetching_page_title(url: &str) -> String {
    String::from("")
}

pub fn write_to_md_file(file_name: &str, text: String) -> anyhow::Result<()> {
    let mut new_file = generate_writable_file(file_name, "md")?;
    new_file.write_all(text.as_bytes())?;
    Ok(())
}

pub fn write_to_epub(file_name: &str, text: &str, title: &str) -> Result<(), Box<dyn Error>> {
    let mut epub_builder = EpubBuilder::new(ZipLibrary::new().unwrap()).unwrap();
    epub_builder.metadata("author", "docbaygio")?;
    epub_builder.metadata("title", title)?;
    // Set some metadata
    epub_builder.add_content(EpubContent::new("post.xhtml", text.as_bytes())
                     .title("Post")
                     .reftype(ReferenceType::Text))?
        .inline_toc();

    // Write EPUB to file
    let mut epub_file = fs::File::create(format!("{}.epub", file_name))?;
    epub_builder.generate(&mut epub_file)?;
    Ok(())
}

pub fn generate_writable_file(file_name: &str, file_format: &str) -> Result<File, std::io::Error> {
    OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(format!("{}.{}", file_name, file_format))
}

pub fn gen_urls(text_file: &Path) -> anyhow::Result<Vec<String>> {
    let links: Vec<String> = fs::read_to_string(text_file)?.lines().map(|line| String::from(line)).collect();
    Ok(links)
}

