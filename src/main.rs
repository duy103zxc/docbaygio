use args::get_args;
use content::app;
mod content;
mod utils;
mod args;


fn main() -> anyhow::Result<()> {
    let user_args = get_args(String::from("en"))?;
    app(user_args.ebook_format, user_args.downloading_mode, &user_args.book_lang)
}


