use std::os::unix::process::CommandExt;
use std::process::Command;

// chrono needs this to call Local.datetime_from_str().
use chrono::offset::TimeZone;

const SQLITE3: &'static str = "/usr/bin/sqlite3";
const DB_PATH: &'static str = "/home/kalvin/Documents/personal/expenditures.db";

// Legacy names that don't indicate denomination. USD is the default.
const PEREXP: &'static str = "perexp";
const ESSEXP: &'static str = "essexp";

// Newer names that indicate denomination.
const JP_PERSONAL: &'static str = "jp_personal";
const JP_ESSENTIAL: &'static str = "jp_essential";

const CLAP_AMOUNT: &'static str = "amount";
const CLAP_DESCRIPTION: &'static str = "description";
const CLAP_EXPENDITURE_TYPE: &'static str = "expenditure-type";
const CLAP_TARGET_DATE: &'static str = "target-date";

#[derive(Debug)]
pub enum ExpenditureType {
    PersonalUSD,
    EssentialUSD,
    PersonalJPY,
    EssentialJPY,
}

#[derive(Debug)]
pub struct SqlOptions {
    // Expenditure type is always required in transacting expenditures.
    expenditure_type: ExpenditureType,

    // Target date is always required in transacting expenditures.
    target_date: chrono::Date<chrono::Local>,

    // Amount and description are required for insertion.
    // They are meaningless for read-only queries.
    amount: Option<f64>,
    description: Option<String>,
}

#[derive(Debug)]
pub enum SqlCommand {
    Edit,
    Show(SqlOptions),
    Insert(SqlOptions),
}

mod from_clap {
    use super::*;

    pub fn expenditure_type(matches: &clap::ArgMatches) -> ExpenditureType {
        let symbolic_type = matches.get_one::<String>(CLAP_EXPENDITURE_TYPE).unwrap();

        match symbolic_type.as_str() {
            "up" => return ExpenditureType::PersonalUSD,
            "ue" => return ExpenditureType::EssentialUSD,
            "jp" => return ExpenditureType::PersonalJPY,
            "je" => return ExpenditureType::EssentialJPY,
            _ => (),
        };
        panic!("invalid {} ``{}''", CLAP_EXPENDITURE_TYPE, symbolic_type);
    }

    pub fn target_date(matches: &clap::ArgMatches) -> Option<chrono::Date<chrono::Local>> {
        if let Some(cli_target_date) = matches.get_one::<String>(CLAP_TARGET_DATE) {
            if let Ok(datetime) = chrono::Local.datetime_from_str(
                &format!("{} 00:00:00", cli_target_date).to_string(),
                "%Y-%m-%d %H:%M:%S",
            ) {
                return Some(datetime.date());
            }
            if let Ok(date_delta) = cli_target_date.parse::<i64>() {
                return Some((chrono::Local::now() - chrono::Duration::days(date_delta)).date());
            }
            panic!("bad target date: ``{}''", cli_target_date);
        }
        None
    }

    pub fn amount(matches: &clap::ArgMatches) -> f64 {
        matches.get_one::<f64>(CLAP_AMOUNT).unwrap().to_owned()
    }

    pub fn description(matches: &clap::ArgMatches) -> String {
        matches
            .get_one::<String>(CLAP_DESCRIPTION)
            .unwrap()
            .to_owned()
    }
} // mod from_clap

fn build_show_options(matches: &clap::ArgMatches) -> SqlOptions {
    let target_date = match from_clap::target_date(matches) {
        Some(date) => date,
        // Aribtrary choice: peeks back 6 months.
        None => (chrono::Local::now() - chrono::Duration::days(183)).date(),
    };

    SqlOptions {
        expenditure_type: from_clap::expenditure_type(matches),
        target_date: target_date,
        amount: None,
        description: None,
    }
}

fn build_insert_options(matches: &clap::ArgMatches) -> SqlOptions {
    let target_date = match from_clap::target_date(matches) {
        Some(date) => date,
        None => chrono::Local::now().date(),
    };

    SqlOptions {
        expenditure_type: from_clap::expenditure_type(matches),
        target_date: target_date,
        amount: Some(from_clap::amount(matches)),
        description: Some(from_clap::description(matches)),
    }
}

fn parse_clap_matches(matches: clap::ArgMatches) -> SqlCommand {
    match matches.subcommand() {
        Some(("edit", _)) => return SqlCommand::Edit,
        Some(("show", show_matches)) => return SqlCommand::Show(build_show_options(show_matches)),
        Some(("insert", insert_matches)) => {
            return SqlCommand::Insert(build_insert_options(insert_matches))
        }
        Some((&_, _)) | None => panic!("no subcommand"),
    }
}

fn parse_command_line() -> SqlCommand {
    let matches = clap::command!()
        .version("0.1.1")
        .author("j39m")
        .about("manipulates expenditures")
        .subcommand(clap::Command::new("edit").about("opens sqlite3 directly"))
        .subcommand(
            clap::Command::new("show")
                .about("shows expenditures")
                .arg(clap::Arg::new(CLAP_TARGET_DATE).short('t'))
                .arg(clap::Arg::new(CLAP_EXPENDITURE_TYPE).required(true)),
        )
        .subcommand(
            clap::Command::new("insert")
                .about("inserts an expenditure")
                .arg(
                    clap::Arg::new(CLAP_AMOUNT)
                        .required(true)
                        .short('a')
                        .value_parser(clap::value_parser!(f64)),
                )
                .arg(clap::Arg::new(CLAP_DESCRIPTION).required(true).short('d'))
                .arg(clap::Arg::new(CLAP_TARGET_DATE).short('t'))
                .arg(clap::Arg::new(CLAP_EXPENDITURE_TYPE).required(true)),
        )
        .get_matches();

    parse_clap_matches(matches)
}

fn expenditure_type_name_from_enum(type_: &ExpenditureType) -> &'static str {
    match type_ {
        ExpenditureType::PersonalUSD => return PEREXP,
        ExpenditureType::EssentialUSD => return ESSEXP,
        ExpenditureType::PersonalJPY => return JP_PERSONAL,
        ExpenditureType::EssentialJPY => return JP_ESSENTIAL,
    }
}

fn build_show_command(options: SqlOptions) -> String {
    let wordy_display_command = format!(
        r#"select {} from {} where date >= date("{}");"#,
        "*",
        expenditure_type_name_from_enum(&options.expenditure_type),
        options.target_date.format("%Y-%m-%d").to_string()
    );
    let summation_command = format!(
        r#"select {} from {} where date >= date("{}");"#,
        "sum(amount)",
        expenditure_type_name_from_enum(&options.expenditure_type),
        options.target_date.format("%Y-%m-%d").to_string()
    );

    format!("{} {}", wordy_display_command, summation_command)
}

fn build_insert_command(options: SqlOptions) -> String {
    format!(
        r#"insert into {} values(date("{}"), "{}", {});"#,
        expenditure_type_name_from_enum(&options.expenditure_type),
        options.target_date.format("%Y-%m-%d"),
        options.description.unwrap(),
        options.amount.unwrap()
    )
}

fn build_extra_sqlite_arg() -> Option<String> {
    let sql_command = parse_command_line();

    match sql_command {
        SqlCommand::Edit => return None,
        SqlCommand::Show(options) => return Some(build_show_command(options)),
        SqlCommand::Insert(options) => return Some(build_insert_command(options)),
    };
}

fn main() {
    let mut command = Command::new(SQLITE3);
    command.arg(DB_PATH);

    if let Some(extra_arg) = build_extra_sqlite_arg() {
        command.arg(extra_arg);
    }

    eprintln!("{:#?}", command);
    command.exec();
}
