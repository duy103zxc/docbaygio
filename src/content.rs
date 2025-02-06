use nanohtml2text::html2text;
use sanitize_html::{rules::predefined::BASIC, sanitize_str};
use epub_builder::{EpubBuilder, EpubContent, ReferenceType, ZipLibrary};
use std::{error::Error, fs, io::Write};
use crate::utils::{self, fetching_page_title};

pub enum RequestedEbookFormat {
    EPUB,
    SingleMarkdown,
    MultipleMarkdown,
    Other
}

pub enum DownloadingMode {
    SingleURL(String),
    MultipleURL(Vec<String>),
    Unknown
}

/*
Lấy HTML Response và chuyển đổi về dạng đọc được
*/

fn clean_raw_response(html_input: &str) -> String {
    sanitize_str(&BASIC, html_input).unwrap()
}

/*
Lấy HTML từ URL và trả về dạng HTML được tinh gọn
*/
fn strip_html(url: &str) -> anyhow::Result<String> {
    Ok(clean_raw_response(
        &crate::utils::fetching_content_from_url(url)?
        .body_mut().read_to_string()?))
}

/*
Tạo tệp XHTML để sử dụng trong khi tạo EPUB
*/
pub fn compose_xhtml(html_input: &str, title: &str, lang: &str) -> String {
    format!(r##"<?xml version='1.0' encoding='utf-8'?>
<html xmlns="http://www.w3.org/1999/xhtml" lang="{}" xml:lang="{}">
    <head>
        <title>{}</title>
        <meta http-equiv="Content-Type" content="text/html; charset=utf-8"/>
        <link rel="stylesheet" type="text/css" href="stylesheet.css"/>
    </head>
    <body>{}{}"##, lang, lang, title, html_input, "\n\t</body>\n</html>")
}

/* Tạo tệp Markdown từ HTML đã được tinh gọn
*/
fn to_markdown(html: &str) -> String {
    html2text(&clean_raw_response(html))
}

pub fn write_to_md_file(file_name: &str, text: String) -> anyhow::Result<()> {
    let mut new_file = utils::generate_writable_file(file_name, "md")?;
    new_file.write_all(text.as_bytes())?;
    Ok(())
}

pub fn single_post_epub(file_name: &str, text: &str) -> Result<(), Box<dyn Error>> {
    let mut epub_builder = EpubBuilder::new(ZipLibrary::new().unwrap()).unwrap();
    epub_builder.metadata("author", "docbaygio")?;
    epub_builder.metadata("title", file_name)?;
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

pub fn write_to_single_epub(posts: Vec<String>) -> Result<(), Box<dyn Error>> {
    let mut epub_builder = EpubBuilder::new(ZipLibrary::new().unwrap()).unwrap();
    epub_builder.metadata("author", "docbaygio")?;
    epub_builder.metadata("title", "EPUB Feed")?;
    // Set some metadata
    for post in posts.iter().enumerate() {
        epub_builder.add_content(EpubContent::new(format!("{}.xhtml", post.0), post.1.as_bytes())
                     .title(format!("Post #{}", post.0))
                     .reftype(ReferenceType::Text))?
        .inline_toc();
    }

    // Write EPUB to file
    let mut epub_file = fs::File::create("EPUB_FEED.epub")?;
    epub_builder.generate(&mut epub_file)?;
    Ok(())
}
/* The main function
Các chế độ nén:
Người dùng sẽ có thể tải xuống một bài, hoặc nhiều bài

- Tải Markdown:
    - Nếu người dùng chỉ để 1 đường dẫn
    - Nếu người dùng để nhiều đường dẫn:
        - Nhiều tệp Markdown
        - Một tệp Markdown duy nhất 
- Tải EPUB:
    - Nếu người dùng chỉ để 1 đường dẫn -> Một tệp EPUB
    - Nếu người dùng để nhiều đường dẫn -> Nén tất cả vào một tệp EPUB:
        - Tiêu đề do người dùng đặt
        - Tác giả là `docbaygio`
        - Generator cũng là `docbaygio`
*/

pub fn app(user_requested_format: RequestedEbookFormat, downloading_mode: DownloadingMode, lang: &str) -> anyhow::Result<()> {
    match downloading_mode {
        DownloadingMode::SingleURL(item) => {
            match user_requested_format {
                RequestedEbookFormat::EPUB => {
                    single_post_epub(
                        &fetching_page_title(&item),
                        &compose_xhtml(
                            &strip_html(&item)?, 
                            &fetching_page_title(&item), 
                            lang)
                        ).unwrap();
                },
                RequestedEbookFormat::SingleMarkdown => {
                    write_to_md_file(
                        &fetching_page_title(&item), 
                        to_markdown(&strip_html(&item)?))?;
                },
                RequestedEbookFormat::MultipleMarkdown => {
                    println!("Doesn't support multiple file for SINGLEURL");
                },
                RequestedEbookFormat::Other => {
                    println!("Unknown e-book format");
                },
                
            }
        },
        DownloadingMode::MultipleURL(items) => {
            match user_requested_format {
                RequestedEbookFormat::EPUB => {
                    write_to_single_epub(items.iter().map(|link| 
                        compose_xhtml(&strip_html(&link).unwrap(), 
                        &fetching_page_title(&link),
                        lang)
                    ).collect::<Vec<String>>()).unwrap();
                },
                RequestedEbookFormat::SingleMarkdown => {
                    write_to_md_file("Feed", items.iter().map(|item| 
                        to_markdown(&strip_html(&item).unwrap())
                    ).collect::<String>())?;
                },
                RequestedEbookFormat::MultipleMarkdown => {
                    for item in items {
                        write_to_md_file(
                            &fetching_page_title(&item), 
                            to_markdown(&strip_html(&item)?))?
                    }
                },
                RequestedEbookFormat::Other => {
                    println!("Unknown e-book format");
                },
            }
        },
        DownloadingMode::Unknown => {
            println!("Unknown downloading mode");
        }
    };
    Ok(())
    

}