# Một ứng dụng giúp tải xuống nhanh một trang dưới dạng Markdown, và tạo thư mục cho ảnh bất kì

### Ý tưởng dự án

1. Dự án của bạn là gì và về cái gì? - Ứng dụng tương tự papeer nhưng được viết bằng Rust, có khả năng tải xuống nội dung từ trình duyệt về để đọc.

2. MVP (Minimal Viable Product hay "Sản phẩm khả thi tối thiểu") của dự án đó là gì? - Một ứng dụng CLI có khả năng tải xuống một hoặc nhiều bài viết về dưới dạng Markdown hoặc Epub.

4. Khi nào dự án sẽ hoàn thành? - Khi hoàn thành MVP


### Bảng Kanban
Lần này hãy thử viết nó thành một Library đi :D

#### TODO (Cần làm)


- Tạo một trait để xử lý vụ này có được k?
#### DOING (Đang làm)

#### DONE (Đã làm xong)
- Fetch content from browser - MVP -> strip content to pure html - MVP Chỉ sử dụng hai function cho cả hai cái này
- tải một bài viêt 
    - Convert from HTML to Markdown - MVP -> pub fn to_markdown(content: &str) -> &str (return corresponding markdown content)
    - Convert from HTML to EPUB - MVP -> pub fn to_epub() (chưa xử lý được), helper functions:
        - pub fn to_xhtml(content: &str, lang: &str) -> &str (Tạo một tệp xhtml cho từng bài đăng một)
        - Xử lý việc nén nội dung vào EPUB trong epub_builder
- Tải nhiều bài viết
    - Thêm -a tag để xử lý links từ tệp text -> read_links_from_text_file(path: &str) -> anyhow::Result<(Vec<String>)>
    - xử lý tương tự sử dụng "tải một bài viêt"
        - Markdown: Gộp thành một tệp Markdown hoặc chia ra làm nhiều tệp
        - EPUB: Cái này khó hơn chút

#### BUGS / NOT SURE HOW TO DO (Lỗi, không biết nên làm gì)

#### Những nhiệm vụ chưa cần tập trung làm
- Download images to the images/ folder - MVP - Để cuối
- Xử lý argument sử dụng clap - Cân nhắc cuối cùng (Đây là nhiệm vụ cần làm)


### Danh sách các functions + datatype cho input và output


### main problems

#### Download images to the images/ folder
using ureq to fetch the content, 

#### 3. Các ý tưởng bất chợt
- Tạo một thư mục, xử lý toàn bộ các nội dung trong đó thay vì lưu trong bộ nhớ đệm
- Có tệp nào để kiểm tra tiến trình không?

### additional

Viết ví dụ cho lệnh

docbaygio get "post-link"
docbaygio get -a (--file) post_link_file