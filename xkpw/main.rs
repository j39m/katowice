extern crate clap;

use std::result::Result;
use std::string::String;
use std::vec::Vec;

mod structs {

    pub const DEFAULT_ENGLISH_DICTIONARY_PATH: &'static str = "/usr/share/dict/words";

    #[derive(Debug)]
    pub enum XkpwError {
        // Denotes an invalid command-line argument.
        InvalidArgument(String),
    }

    impl std::fmt::Display for XkpwError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                XkpwError::InvalidArgument(message) => write!(f, "{}", message),
            }
        }
    }

    #[derive(Debug)]
    pub struct EnglishPasswordOptions {
        pub num_words: u8,
        pub dictionary_path: String,
    }

    #[derive(Debug)]
    pub struct KanaPasswordOptions {
        pub syllable_counts: Vec<u8>,
    }

    pub enum PasswordOptions {
        English(EnglishPasswordOptions),
        Japanese(KanaPasswordOptions),
    }
} // mod structs

mod args {
    use crate::structs::{EnglishPasswordOptions, KanaPasswordOptions, PasswordOptions, XkpwError};
    use clap::{value_t, values_t};

    // Aborts this process on error.
    pub fn parse_args() -> PasswordOptions {
        let matches = clap::App::new("xkpw")
            .version("0.1.0")
            .author("j39m")
            .about("Generates passwords")
            .subcommand(
                clap::SubCommand::with_name("en")
                    .about("generates dictionary passwords in English")
                    .arg(clap::Arg::with_name("num-words").required(true)),
            )
            .subcommand(
                clap::SubCommand::with_name("jp")
                    .about("generates random strings of Japanese syllables")
                    .arg(
                        clap::Arg::with_name("syllable-counts")
                            .required(true)
                            .multiple(true),
                    ),
            )
            .get_matches();

        match matches.subcommand() {
            ("en", Some(en_matches)) => {
                return PasswordOptions::English(EnglishPasswordOptions {
                    num_words: clap::value_t!(en_matches, "num-words", u8)
                        .unwrap_or_else(|e| e.exit()),
                    dictionary_path: crate::structs::DEFAULT_ENGLISH_DICTIONARY_PATH.to_owned(),
                })
            }

            ("jp", Some(jp_matches)) => {
                return PasswordOptions::Japanese(KanaPasswordOptions {
                    syllable_counts: clap::values_t!(jp_matches, "syllable-counts", u8)
                        .unwrap_or_else(|e| e.exit()),
                })
            }
            _ => ()
        }
        PasswordOptions::Japanese(KanaPasswordOptions {
            syllable_counts: vec![4, 3, 3, 3],
        })
    }
} // mod args

fn main_helper() -> i32 {
    let options = crate::args::parse_args();
    1
}

fn main() {
    std::process::exit(main_helper());
}
