# docbaygio
Ứng dụng CLI giúp tải toàn bộ bài viết và nén thành một tệp EPUB hoàn chỉnh. Có thể sẽ hỗ trợ Markdown với PDF trong thời gian tới nếu không lười.

### Tải ứng dụng
Bạn ghé vào phần [Release](https://github.com/duy103zxc/docbaygio/releases) để tải xuống (Hiện tại chưa làm bản cho Windows).

### Hướng dẫn sử dụng
Chương trình sẽ có hai chế độ tải xuống, tải thông qua đường dẫn và thông qua tệp txt: 

- `get-file`: Sẽ nhập tệp `txt` làm Input.
- `get-link`: Sẽ nhập đường dẫn URL làm Input.

Bạn cần phải nhập thêm cả mã ngôn ngữ. Đọc thêm các mã ngôn ngữ được hỗ trợ trong phần mềm tại: [Những ngôn ngữ được hỗ trợ](supported_languages.txt). Bạn sẽ phải nhập đúng cú pháp để tải được. 
Bạn chỉ cần nhập:

```bash
docbaygio [lệnh tải] [ngôn ngữ] [đường dẫn bài viết/tệp lưu các đường dẫn]
# Ví dụ
docbaygio get-link en https://jdemeta.net/2019/09/15/exiting-modernity/
docbaygio get-file vi links.txt
```

**Chế độ 1**: Tải một bài viết thông qua đường dẫn:

```bash
docbaygio get-link en    https://jdemeta.net/2019/09/15/exiting-modernity/
^         ^        ^      ^
Tên ứng | Lệnh   | Ngôn | Đường dẫn bài viết
dụng      tải      ngữ
```

**Chế độ 2**: Tải nhiều bài viết thông qua một tệp txt:

Nếu bạn muốn tải nhiều bài viết cùng lúc và đưa nó vào trong một tệp EPUB thì làm theo hướng dẫn này.

Bạn tạo một tệp `txt`, ví dụ như `links.txt`, tại thư mục bạn cần tải bài viết, rồi thêm các đường dẫn vào trong tệp, mỗi đường dẫn cách một dòng, đây là một ví dụ:

```
https://every.to/superorganizers/how-to-become-indistractable-in-2025
https://every.to/superorganizers/how-to-make-yourself-into-a-learning-machine
```
Sau đó chạy lệnh dưới đây

```bash
docbaygio get-file en    links.txt
^         ^        ^      ^
Tên ứng | Lệnh   | Ngôn | Tên tệp lưu trữ đường dẫn
dụng      tải      ngữ
```

Rồi chờ là xong!

### Build và Test ứng dụng

```bash
# Phần này sẽ tạo bản release cho ứng dụng và chuyển vào thư mục ~/.local/bin/ trong Linux
./build.sh
# Chạy kiểm tra xem ứng dụng hoạt động ổn áp không
./test.sh
```