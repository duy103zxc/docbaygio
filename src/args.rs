use anyhow::Ok;
use clap::Parser;

use crate::{content::{DownloadingMode, RequestedEbookFormat}, utils::gen_urls};
// Sẽ đổi qua sử dụng clap do tính chất phức tạp của phần mềm

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(short, long)]
    mode: String,
    #[arg(short, long)]
    format: String,
    #[arg(short, long)]
    content: String
}

pub struct ParsedArgs {
    pub downloading_mode: DownloadingMode,
    pub ebook_format: RequestedEbookFormat,
    pub book_lang: String
}
pub fn get_args(language: String) -> anyhow::Result<ParsedArgs> {
    let args = Args::parse();

    let arg_mode = match args.mode.as_str() {
        "single" => DownloadingMode::SingleURL(args.content),
        "multiple" => DownloadingMode::MultipleURL(gen_urls(&args.content)?),
        _ => DownloadingMode::Unknown
    };

    let book_format = match args.format.as_str() {
        "epub" => RequestedEbookFormat::EPUB,
        "md" => RequestedEbookFormat::SingleMarkdown,
        "multiple_md" => RequestedEbookFormat::MultipleMarkdown,
        _ => RequestedEbookFormat::Other
    };
    Ok(ParsedArgs {
        downloading_mode: arg_mode,
        ebook_format: book_format,
        book_lang: language,
    })
}