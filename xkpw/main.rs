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
    use super::{
        EnglishPasswordOptions, KanaPasswordOptions, PasswordOptions,
        DEFAULT_ENGLISH_DICTIONARY_PATH,
    };

    // Aborts this process on error.
    pub fn parse_args() -> PasswordOptions {
        let matches = clap::App::new("xkpw")
            .version("0.1.1")
            .author("j39m")
            .about("Generates passwords")
            .subcommand(
                clap::App::new("en")
                    .about("generates dictionary passwords in English")
                    .arg(clap::Arg::new("num-words").required(true)),
            )
            .subcommand(
                clap::App::new("jp")
                    .about("generates random strings of Japanese syllables")
                    .arg(
                        clap::Arg::new("syllable-counts")
                            .required(true)
                            .multiple_values(true),
                    ),
            )
            .get_matches();

        match matches.subcommand() {
            Some(("en", en_matches)) => {
                return PasswordOptions::English(EnglishPasswordOptions {
                    num_words: en_matches
                        .value_of_t::<u8>("num-words")
                        .unwrap_or_else(|e| e.exit()),
                    dictionary_path: DEFAULT_ENGLISH_DICTIONARY_PATH.to_owned(),
                })
            }
            Some(("jp", jp_matches)) => {
                return PasswordOptions::Japanese(KanaPasswordOptions {
                    syllable_counts: jp_matches
                        .values_of_t("syllable-counts")
                        .unwrap_or_else(|e| e.exit()),
                })
            }
            _ => (),
        }
        PasswordOptions::Japanese(KanaPasswordOptions {
            syllable_counts: vec![4, 3, 3, 3],
        })
    }
} // mod args

mod kana {
    #[rustfmt::skip]
    const BASE: &'static [&str] = &[
        "a",    "i",    "u",    "e",    "o",
        "ka",   "ki",   "ku",   "ke",   "ko",
        "sa",   "shi",  "su",   "se",   "so",
        "ta",   "chi",  "tsu",  "te",   "to",
        "na",   "ni",   "nu",   "ne",   "no",
        "ha",   "hi",   "hu",   "he",   "ho",
        "ma",   "mi",   "mu",   "me",   "mo",
        "ya",           "yu",           "yo",  
        "ra",   "ri",   "ru",   "re",   "ro",
        "wa",                           "wo",
    ];

    #[rustfmt::skip]
    const DIGRAPHS: &'static [&str] = &[
        "kya", "kyu", "kyo",
        "sha", "shu", "sho",
        "cha", "chu", "cho",
        "nya", "nyu", "nyo",
        "hya", "hyu", "hyo",
        "mya", "myu", "myo",
        "rya", "ryu", "ryo",
    ];

    #[rustfmt::skip]
    const DIACRITICS: &'static [&str] = &[
        "ga", "gi", "gu", "ge", "go",
        "za", "ji", "zu", "ze", "zo",
        "da",             "de", "do",
        "ba", "bi", "bu", "be", "bo",
        "pa", "pi", "pu", "pe", "po",
    ];

    #[rustfmt::skip]
    const DIGRAPHS_WITH_DIACRITICS: &'static [&str] = &[
        "gya",  "gyu",  "gyo",
        "ja",   "ju",   "jo",
        "bya",  "byu",  "byo",
        "pya",  "pyu",  "pyo",
    ];

    fn all() -> Vec<&'static str> {
        let mut all_kana: Vec<&'static str> = Vec::new();
        all_kana.extend(BASE);
        all_kana.extend(DIGRAPHS);
        all_kana.extend(DIACRITICS);
        all_kana.extend(DIGRAPHS_WITH_DIACRITICS);
        all_kana
    }

    fn simple() -> Vec<&'static str> {
        let mut simple_kana: Vec<&'static str> = Vec::new();
        simple_kana.extend(BASE);
        simple_kana.extend(DIACRITICS);
        simple_kana
    }

    pub fn get(use_simple: bool) -> Vec<&'static str> {
        match use_simple {
            true => simple(),
            false => all(),
        }
    }
} // mod kana

mod helpers {
    use super::{EnglishPasswordOptions, KanaPasswordOptions, PasswordOptions};
    use rand::seq::SliceRandom;
    use std::convert::TryInto;
    use std::io::Read;

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

    // Assumes ownership of a Vector of owned |words| and prints them
    // separated by spaces.
    fn print_words(words: Vec<String>) {
        let mut iter = words.into_iter();
        if let Some(word) = iter.next() {
            print!("{}", word);
            for word in iter {
                print!(" {}", word);
            }
            print!("\n");
        }
    }

    // Returns 1 if we fail to read the word dictionary.
    fn print_english_password(options: EnglishPasswordOptions) -> i32 {
        let all_words =
            match ingest_english_dictionary(std::path::Path::new(&options.dictionary_path)) {
                Ok(result) => result,
                Err(error) => {
                    eprintln!("Error reading dictionary: {}", error);
                    return 1;
                }
            };
        let mut rng = &mut rand::thread_rng();
        let selection: Vec<String> = all_words
            .choose_multiple(&mut rng, options.num_words.try_into().unwrap())
            .map(|borrowed| borrowed.to_owned())
            .collect();

        print_words(selection);
        0
    }

    // Builds a single random pseudo-Japanese word from |kana_set|.
    fn build_kana_word(kana_set: &Vec<&'static str>, syllable_count: &u8) -> String {
        let mut collected_kana: Vec<&'static str> = Vec::new();
        let mut rng = &mut rand::thread_rng();
        for _ in 0..*syllable_count {
            collected_kana.push(kana_set.choose(&mut rng).unwrap());
        }
        collected_kana.join("")
    }

    // Builds random pseudo-Japanese words from |kana_set|, observing
    // the word lengths specified by |options|.
    fn build_kana_words(kana_set: Vec<&'static str>, options: KanaPasswordOptions) -> Vec<String> {
        options
            .syllable_counts
            .iter()
            .map(|count| build_kana_word(&kana_set, count))
            .collect()
    }

    // Builds a vector of owned kana words, maps these to a vector of
    // borrowed kana words, and then prints the same.
    fn print_kana_password(options: KanaPasswordOptions) {
        print_words(build_kana_words(crate::kana::get(true), options));
    }

    pub fn main() -> i32 {
        let options = crate::args::parse_args();
        match options {
            PasswordOptions::English(options) => return print_english_password(options),
            PasswordOptions::Japanese(options) => print_kana_password(options),
        }
        0
    }
} // mod helpers

fn main() {
    std::process::exit(crate::helpers::main());
}
