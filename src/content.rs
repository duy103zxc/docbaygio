use nanohtml2text::html2text;
use sanitize_html::{rules::predefined::BASIC, sanitize_str};

use crate::utils::{fetching_page_title, write_to_md_file};

pub enum RequestedEbookFormat {
    EPUB,
    SingleMarkdown,
    MultipleMarkdown,
    Other
}

pub enum DownloadingMode {
    SingleURL(String),
    MultipleURL(Vec<String>)
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

pub fn app(user_requested_format: RequestedEbookFormat, downloading_mode: DownloadingMode) -> anyhow::Result<()> {
    match downloading_mode {
        DownloadingMode::SingleURL(item) => {
            match user_requested_format {
                RequestedEbookFormat::EPUB => todo!(),
                RequestedEbookFormat::SingleMarkdown => {
                    write_to_md_file(&fetching_page_title(&item), to_markdown(&strip_html(&item)?));
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
                RequestedEbookFormat::EPUB => todo!(),
                RequestedEbookFormat::SingleMarkdown => todo!(),
                RequestedEbookFormat::MultipleMarkdown => todo!(),
                RequestedEbookFormat::Other => todo!(),
            }
        },
    };
    Ok(())
    

}