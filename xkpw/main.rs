extern crate clap;
extern crate rand;

pub const DEFAULT_ENGLISH_DICTIONARY_PATH: &'static str = "/usr/share/dict/words";

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

mod args {
    use crate::{EnglishPasswordOptions, KanaPasswordOptions, PasswordOptions};
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
                return crate::PasswordOptions::English(crate::EnglishPasswordOptions {
                    num_words: clap::value_t!(en_matches, "num-words", u8)
                        .unwrap_or_else(|e| e.exit()),
                    dictionary_path: crate::DEFAULT_ENGLISH_DICTIONARY_PATH.to_owned(),
                })
            }
            ("jp", Some(jp_matches)) => {
                return crate::PasswordOptions::Japanese(crate::KanaPasswordOptions {
                    syllable_counts: clap::values_t!(jp_matches, "syllable-counts", u8)
                        .unwrap_or_else(|e| e.exit()),
                })
            }
            _ => ()
        }
        crate::PasswordOptions::Japanese(crate::KanaPasswordOptions {
            syllable_counts: vec![4, 3, 3, 3],
        })
    }
} // mod args

mod helpers {
    use rand::seq::SliceRandom;
    use std::io::Read;
    use std::convert::TryInto;

    // Reads the dictionary at |path| and returns a linewise vector of
    // its contents.
    fn ingest_english_dictionary(path: &std::path::Path) -> std::io::Result<Vec<String>> {
        let mut file = std::fs::File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents
            .lines()
            .map(|word| word.trim().to_lowercase())
            .collect())
    }

    // Returns 1 if we fail to read the word dictionary.
    fn print_english_password(options: crate::EnglishPasswordOptions) -> i32 {
        let words = match ingest_english_dictionary(std::path::Path::new(&options.dictionary_path))
        {
            Ok(result) => result,
            Err(error) => {
                eprintln!("Error reading dictionary: {}", error);
                return 1;
            }
        };
        let mut rng = &mut rand::thread_rng();
        let selection: Vec<&String> = words
            .choose_multiple(&mut rng, options.num_words.try_into().unwrap())
            .collect();

        let mut iter = selection.iter();
        if let Some(word) = iter.next() {
            print!("{}", word);
            for word in iter {
                print!(" {}", word);
            }
            print!("\n");
        }
        0
    }

    fn print_kana_password(options: crate::KanaPasswordOptions) {
        eprintln!("XXX(j39m): FIX ME");
    }

    pub fn main() -> i32 {
        let options = crate::args::parse_args();
        match options {
            crate::PasswordOptions::English(options) => return print_english_password(options),
            crate::PasswordOptions::Japanese(options) => print_kana_password(options),
        }
        0
    }
} // mod helpers

fn main() {
    std::process::exit(crate::helpers::main());
}
