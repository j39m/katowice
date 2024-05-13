const DEFAULT_ENGLISH_DICTIONARY_PATH: &'static str = "/usr/share/dict/words";

use clap::Parser;

#[derive(clap::Parser)]
#[command(name = clap::crate_name!(), version = clap::crate_version!())]
struct Cli {
    #[command(subcommand)]
    subcommand: Option<Subcommand>,
}

#[derive(clap::Subcommand)]
enum Subcommand {
    /// create English-ish password
    En(EnglishArgs),
    /// create faux-Japanese password
    Jp(JapaneseArgs),
}

#[derive(clap::Args)]
struct EnglishArgs {
    #[arg(short, default_value_t =
          String::from(DEFAULT_ENGLISH_DICTIONARY_PATH),
          help = "dictionary path")]
    dictionary: String,
    #[arg(help = "word count")]
    count: usize,
}

#[derive(clap::Args)]
struct JapaneseArgs {
    #[arg(short, default_value_t = false, help = "also use harder kana")]
    all_kana: bool,
    #[arg(default_values_t = vec![4, 3, 3, 3], help = "syllable counts")]
    counts: Vec<usize>,
}

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

    pub fn get(use_all_hiragana: bool) -> Vec<&'static str> {
        match use_all_hiragana {
            true => all(),
            false => simple(),
        }
    }
} // mod kana

use rand::seq::SliceRandom;
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
fn print_english_password(args: EnglishArgs) {
    let all_words = match ingest_english_dictionary(std::path::Path::new(&args.dictionary)) {
        Ok(result) => result,
        Err(error) => {
            panic!("Error reading dictionary: {}", error);
        }
    };
    let mut rng = &mut rand::thread_rng();
    let selection: Vec<String> = all_words
        .choose_multiple(&mut rng, args.count)
        .map(|borrowed| borrowed.to_owned())
        .collect();

    print_words(selection);
}

// Builds a single random pseudo-Japanese word from |kana_set|.
fn build_kana_word(kana_set: &Vec<&'static str>, syllable_count: usize) -> String {
    let mut collected_kana: Vec<&'static str> = Vec::new();
    let mut rng = &mut rand::thread_rng();
    for _ in 0..syllable_count {
        collected_kana.push(kana_set.choose(&mut rng).unwrap());
    }
    collected_kana.join("")
}

// Builds a vector of owned kana words, maps these to a vector of
// borrowed kana words, and then prints the same.
fn print_kana_password(args: JapaneseArgs) {
    let syllable_set = crate::kana::get(args.all_kana);
    let words = args
        .counts
        .into_iter()
        .map(|morae_count| build_kana_word(&syllable_set, morae_count))
        .collect();
    print_words(words);
}

fn main() {
    let cli = Cli::parse();
    match cli.subcommand {
        Some(Subcommand::En(args)) => print_english_password(args),
        Some(Subcommand::Jp(args)) => print_kana_password(args),
        None => print_kana_password(JapaneseArgs {
            all_kana: true,
            counts: vec![4, 3, 3, 3],
        }),
    };
}
