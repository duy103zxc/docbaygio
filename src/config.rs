use crate::epub::check_valid_lang_code;
use crate::epub::gen_epub;
use crate::epub::gen_urls;
use std::env;
use std::error::Error;

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
            None => return Err("No command found. Terminated.."),
        };

        let language = match args.next() {
            Some(lang) => check_valid_lang_code(&lang).unwrap(),
            None => String::from("en"),
        };

        // Depended on "command"
        let user_input = match args.next() {
            Some(arg) => arg,
            None => return Err("No source found"),
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
                println!("Something went wrong");
            }
        },
        _ => {
            println!("Unsupported command");
        }
    }

    Ok(())
}
