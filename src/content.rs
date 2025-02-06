
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

pub fn to_markdown(html: &str) -> String {
    html2text(html)
}