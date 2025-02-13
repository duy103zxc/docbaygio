use epub_builder::EpubBuilder;
use epub_builder::EpubContent;
use epub_builder::ReferenceType;
use epub_builder::ZipLibrary;
use readability::extractor;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::path::Path;

pub fn gen_epub(urls: Vec<String>, lang: &str) -> Result<(), Box<dyn Error>> {
    let file_name = &gen_filename();
    let mut epub_builder = EpubBuilder::new(ZipLibrary::new().unwrap()).unwrap();
    epub_builder.metadata("author", "Docbaygio")?;
    epub_builder.metadata("title", file_name)?;
    epub_builder.epub_version(epub_builder::EpubVersion::V30);
    for url in urls.iter().enumerate() {
        let post = extractor::scrape(url.1)?;
        epub_builder
            .add_content(
                EpubContent::new(
                    format!("{}.html", url.0),
                    compose_html(&post.content, &post.title, lang).as_bytes(),
                )
                .title(&post.title)
                .reftype(ReferenceType::Text),
            )?;
    }
    epub_builder.inline_toc();
    // Write EPUB to file
    let mut epub_file = generate_empty_epub_file(file_name)?;
    epub_builder.generate(&mut epub_file)?;
    Ok(())
}

pub fn compose_html(html_input: &str, title: &str, lang: &str) -> String {
    format!(
        r##"<?xml version='1.0' encoding='utf-8'?>
<html xmlns="http://www.w3.org/1999/xhtml" lang="{}" xml:lang="{}">
    <head>
        <title>{}</title>
        <meta http-equiv="Content-Type" content="text/html; charset=utf-8"/>
    </head>
    <body>
        {}
    {}"##,
        lang, lang, title, html_input, "\n\t</body>\n</html>"
    )
}

pub fn gen_urls(text_file: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let links: Vec<String> = fs::read_to_string(text_file)?
        .lines()
        .map(|line| String::from(line))
        .collect();
    Ok(links)
}

pub fn check_valid_lang_code(lang: &str) -> Result<String, Box<dyn Error>> {
    static LANGUAGE_CODES: &[&'static str; 186] = &[
        "aa", "ab", "ae", "af", "ak", "am", "an", "ar", "as", "av", "ay", "az", "ba", "be", "bg",
        "bh", "bi", "bm", "bn", "bo", "br", "bs", "ca", "ce", "ch", "co", "cr", "cs", "cv", "cy",
        "da", "de", "div", "dv", "dz", "ee", "el", "es", "et", "eu", "fa", "ff", "fi", "fj", "fo",
        "fr", "fy", "ga", "gd", "gl", "gn", "gu", "gv", "ha", "he", "hi", "hr", "ht", "hu", "hy",
        "hz", "ia", "id", "ie", "ig", "ik", "in", "io", "is", "it", "iu", "iw", "ja", "ji", "jv",
        "jw", "ka", "kg", "ki", "kj", "kk", "kl", "km", "kn", "ko", "kok", "kr", "ks", "ku", "kv",
        "kw", "ky", "kz", "la", "lb", "lg", "li", "ln", "lo", "ls", "lt", "lu", "lv", "mg", "mh",
        "mi", "mk", "ml", "mn", "mo", "mr", "ms", "mt", "my", "na", "ng", "nl", "no", "nv", "ny",
        "oc", "oj", "or", "os", "pa", "pi", "pl", "pt", "qu", "rm", "rn", "ro", "ru", "rw", "sa",
        "sb", "sc", "sd", "se", "sg", "sh", "si", "sk", "sl", "sm", "sn", "so", "sq", "sr", "ss",
        "st", "su", "sv", "sw", "sx", "syr", "ta", "te", "tg", "th", "ti", "tk", "tl", "tn", "to",
        "tr", "ts", "tt", "tw", "ty", "ug", "uk", "ur", "us", "uz", "ve", "vi", "vo", "wa", "wo",
        "xh", "yi", "yo", "za", "zh", "zu",
    ];

    if LANGUAGE_CODES.contains(&lang) {
        Ok(String::from(lang))
    } else {
        Ok(String::from("en"))
    }
}

fn generate_empty_epub_file(file_name: &str) -> Result<File, std::io::Error> {
    OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(format!("{}.epub", file_name))
}

fn gen_filename() -> String {
    let mut state = Some(0);
    let mut file_name = String::new();

    while let Some(i) = state {
        if Path::new(&format!("Docbaygio{}.epub", i)).exists() {
            println!("Docbaygio{i}.epub exists, generating a new filename");

            state = Some(i + 1);
        } else {
            file_name = format!("Docbaygio{}", i);
            state = None;
        }
    }
    file_name
}
