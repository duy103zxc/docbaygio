use crate::epub::check_valid_lang_code;
use crate::epub::gen_epub;
use crate::epub::gen_urls;
use std::env;
use std::error::Error;

static HELP_MSG: &str = r##"Hướng dẫn sử dụng ứng dụng Docbaygio

## Tóm tắt

- docbaygio get-file vi links.txt (Tải toàn bộ các đường dẫn URL trong tệp links.txt, đặt ngôn ngữ tải là Tiếng Việt).
- docbaygio get-link en https://jdemeta.net/2019/09/15/exiting-modernity/ (Tải một bài viết vào tệp EPUB, đặt ngôn ngữ là Tiếng Anh)

## Chi tiết

Chương trình sẽ có hai chế độ tải xuống, tải thông qua đường dẫn và thông qua tệp txt: 

- `get-file`: Sẽ nhập tệp `txt` làm Input.
- `get-link`: Sẽ nhập đường dẫn URL làm Input.

Bạn cần phải nhập thêm cả mã ngôn ngữ. Đọc thêm các mã ngôn ngữ được hỗ trợ trong phần mềm tại: [Những ngôn ngữ được hỗ trợ](supported_languages.txt). Bạn sẽ phải nhập đúng cú pháp để tải được. 
Bạn chỉ cần nhập:

docbaygio [lệnh tải] [ngôn ngữ] [đường dẫn bài viết/tệp lưu các đường dẫn]

### Ví dụ

---
docbaygio get-link en https://jdemeta.net/2019/09/15/exiting-modernity/
docbaygio get-file vi links.txt
---

### Chế độ 1: Tải một bài viết thông qua đường dẫn:

---
docbaygio get-link en    https://jdemeta.net/2019/09/15/exiting-modernity/
^         ^        ^      ^
Tên ứng | Lệnh   | Ngôn | Đường dẫn bài viết
dụng      tải      ngữ
---


### Chế độ 2: Tải nhiều bài viết thông qua một tệp txt:

Nếu bạn muốn tải nhiều bài viết cùng lúc và đưa nó vào trong một tệp EPUB thì làm theo hướng dẫn này.

Bạn tạo một tệp `txt`, ví dụ như `links.txt`, tại thư mục bạn cần tải bài viết, rồi thêm các đường dẫn vào trong tệp, mỗi đường dẫn cách một dòng, đây là một ví dụ:

---
https://every.to/superorganizers/how-to-become-indistractable-in-2025
https://every.to/superorganizers/how-to-make-yourself-into-a-learning-machine
---

Sau đó chạy lệnh dưới đây:

---
docbaygio get-file en    links.txt
^         ^        ^      ^
Tên ứng | Lệnh   | Ngôn | Tên tệp lưu trữ đường dẫn
dụng      tải      ngữ
---

Cảm ơn bạn đã sử dụng ứng dụng Docbaygio! Mong nó có ích với bạn ^^!
"##;

pub struct Config {
    pub command: String,
    pub user_input: String,
    pub language: String,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        // Including get-file and get-link
        let command = match args.next() {
            Some(arg) => arg,
            None => return Err("Bạn không nhập lệnh gì cả, tự động thoát đây..."),
        };

        let language = match args.next() {
            Some(lang) => check_valid_lang_code(&lang).unwrap(),
            None => String::from("en"),
        };

        // Depended on "command"
        let user_input = match args.next() {
            Some(arg) => arg,
            None => String::from("..."),
        };

        Ok(Config {
            command,
            user_input,
            language: String::from(language),
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content_link = &config.user_input;
    let lang = &config.language;
    match config.command.as_str() {
        "get-link" => {
            gen_epub(vec![String::from(content_link)], lang)?;
        }
        "get-file" => match gen_urls(content_link) {
            Ok(post_urls) => {
                gen_epub(post_urls, lang)?;
            }

            Err(_) => {
                println!("Không tải được, hình như bị lỗi gì ấy");
            }
        },
        "help" => {
            println!("{}", HELP_MSG);
        }
        _ => {
            println!("Bạn đang nhập sai lệnh thì phải, gõ `docbaygio help` để đọc hướng dẫn");
        }
    }

    Ok(())
}
