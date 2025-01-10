# Một ứng dụng giúp tải xuống nhanh một trang dưới dạng Markdown, và tạo thư mục cho ảnh bất kì

## Tham khảo Lapwat (Chỉ cần lấy chức năng `get` là được)

## Thực ra là Lapwat làm được rồi :))

Nhưng còn nếu mà muốn tải nhiều bài viết với cùng một thư mục thì sao

Làm một phần mềm để khi tải thì sẽ trả về 

Tự viết một Library kiểu như go-readability -> Chỉ lấy những tag cần thiết, về cơ bản từ một tệp HTML gốc, mình sẽ cắt hết các elements và class thừa

- Thử nghiệm: https://docs.rs/html-extractor/latest/html_extractor/


Vậy thì cần tóm tắt lại:

- Viết một crate tương tự go-readability => extract content from HTML (https://docs.rs/html-purifier/latest/html_purifier/)
- html-to-markdown => convert HTML to Markdown
- Sử dụng go-epub => convert HTML to EPUB
- colly - query HTML trees -> like scraper
- Sửa lại lỗi xử lý epub trong: https://github.com/duy103zxc/epub-builder 

Mình cần phải chuyển từ Markdown -> HTML -> EPUB


### Ý tưởng dự án

1. Dự án của bạn là gì và về cái gì? - Ứng dụng tương tự papeer nhưng được viết bằng Rust, có khả năng tải xuống nội dung từ trình duyệt về để đọc.

2. MVP (Minimal Viable Product hay "Sản phẩm khả thi tối thiểu") của dự án đó là gì? - Một ứng dụng CLI có khả năng tải xuống một hoặc nhiều bài viết về dưới dạng Markdown hoặc Epub.

3. Sprinkles là gì? - Có thể sử dụng được Class Selector.

4. Khi nào dự án sẽ hoàn thành? - Khi hoàn thành MVP


### Bảng Kanban
Lần này hãy thử viết nó thành một Library đi :D

#### TODO (Cần làm)
- Fetch content from browser - MVP
- strip content to pure html - MVP
- Convert from HTML to Markdown - MVP
- Convert from HTML to EPUB - MVP
- Download images to the images/ folder - MVP
- Xử lý argument - MVP (đơn giản thôi) - Cân nhắc cuối cùng

#### DOING (Đang làm)

#### DONE (Đã làm xong)

#### BUGS / NOT SURE HOW TO DO (Lỗi, không biết nên làm gì)


### main problems

#### Fetch content from browser

using ureq
pub fn reqwester(url: &str, refer_site: &str) -> Result<Response, ureq::Error> {
    let user_agent = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36";
    
    let ureq_agent: Agent = ureq::AgentBuilder::new()
    .timeout_read(Duration::from_secs(30))
    .build();

    let body = ureq_agent.get(url)
        .set("User-Agent", user_agent)
        .set("Referer", refer_site)
        .call();
    body
}

#### strip content to pure html
using https://docs.rs/html-purifier/latest/html_purifier/ (Có thể vấn đề nằm ở cái này)

use html_purifier::{purifier, Settings};

let settings = Settings {
    ..Settings::default()
};
let input = r#"<a href="/test" style="color: black;"><img src="/logo.png" onerror="javascript:;"/>Rust</a>"#;
let output = purifier(input, settings);

#### Convert from HTML to Markdown

https://docs.rs/html2md/latest/html2md/
parse_html(html: &str) -> String

#### Convert from HTML to EPUB

epub-builder = "0.7"


#### Download images to the images/ folder
using ureq to fetch the content, 


