use std::os::unix::process::CommandExt;
use std::process::Command;

// clap exposes the macro value_t!
use clap::value_t;

// chrono needs this to call Utc.datetime_from_str().
use chrono::offset::TimeZone;

const SQLITE3: &'static str = "/usr/bin/sqlite3";
const DB_PATH: &'static str = "/home/kalvin/Documents/personal/expenditures.db";

const PEREXP: &'static str = "perexp";
const ESSEXP: &'static str = "essexp";

const CLAP_AMOUNT: &'static str = "amount";
const CLAP_DESCRIPTION: &'static str = "description";
const CLAP_EXPENDITURE_TYPE: &'static str = "expenditure-type";
const CLAP_TARGET_DATE: &'static str = "target-date";

#[derive(Debug)]
enum ExpenditureType {
    Personal,
    Essential,
}

#[derive(Debug)]
struct SqlOptions {
    // Expenditure type is always required in transacting expenditures.
    expenditure_type: ExpenditureType,

    // Target date is always required in transacting expenditures.
    target_date: chrono::Date<chrono::Utc>,

    // Amount and description are required for insertion.
    // They are meaningless for read-only queries.
    amount: Option<f64>,
    description: Option<String>,
}

#[derive(Debug)]
enum SqlCommand {
    Edit,
    Show(SqlOptions),
    Insert(SqlOptions),
}

fn expenditure_type_from_clap(matches: &clap::ArgMatches) -> ExpenditureType {
    let symbolic_type = matches.value_of(CLAP_EXPENDITURE_TYPE).unwrap();

    match symbolic_type {
        "p" => return ExpenditureType::Personal,
        "e" => return ExpenditureType::Essential,
        _ => (),
    };
    panic!(format!(
        "invalid {} ``{}''",
        CLAP_EXPENDITURE_TYPE, symbolic_type
    ));
}

fn expenditure_target_date_from_clap(
    matches: &clap::ArgMatches,
) -> Option<chrono::Date<chrono::Utc>> {
    if let Some(cli_target_date) = matches.value_of(CLAP_TARGET_DATE) {
        return Some(
            chrono::Utc
                .datetime_from_str(
                    &format!("{} 00:00:00", cli_target_date).to_string(),
                    "%Y-%m-%d %H:%M:%S",
                )
                .unwrap()
                .date(),
        );
    }
    None
}

fn expenditure_amount_from_clap(matches: &clap::ArgMatches) -> f64 {
    clap::value_t!(matches, CLAP_AMOUNT, f64).unwrap_or_else(|e| e.exit())
}

fn expenditure_description_from_clap(matches: &clap::ArgMatches) -> String {
    clap::value_t!(matches, CLAP_DESCRIPTION, String).unwrap_or_else(|e| e.exit())
}

fn build_show_options(matches: &clap::ArgMatches) -> SqlOptions {
    let target_date = match expenditure_target_date_from_clap(matches) {
        Some(date) => date,
        // Aribtrary choice: peeks back 6 months.
        None => (chrono::Utc::now() - chrono::Duration::days(183)).date(),
    };

    SqlOptions {
        expenditure_type: expenditure_type_from_clap(matches),
        target_date: target_date,
        amount: None,
        description: None,
    }
}

fn build_insert_options(matches: &clap::ArgMatches) -> SqlOptions {
    let target_date = match expenditure_target_date_from_clap(matches) {
        Some(date) => date,
        None => chrono::Utc::now().date(),
    };

    SqlOptions {
        expenditure_type: expenditure_type_from_clap(matches),
        target_date: target_date,
        amount: Some(expenditure_amount_from_clap(matches)),
        description: Some(expenditure_description_from_clap(matches)),
    }
}

fn parse_clap_matches(matches: clap::ArgMatches) -> SqlCommand {
    match matches.subcommand() {
        ("edit", _) => return SqlCommand::Edit,
        ("show", Some(show_matches)) => return SqlCommand::Show(build_show_options(show_matches)),
        ("insert", Some(insert_matches)) => {
            return SqlCommand::Insert(build_insert_options(insert_matches))
        }
        (&_, _) => panic!("no subcommand"),
    }
}

fn parse_command_line() -> SqlCommand {
    let matches = clap::App::new("tangerine-tailleferre")
        .version("0.1.0")
        .author("j39m")
        .about("manipulates expenditures")
        .subcommand(clap::App::new("edit").about("opens sqlite3 directly"))
        .subcommand(
            clap::App::new("show")
                .about("shows expenditures")
                .arg(
                    clap::Arg::with_name(CLAP_TARGET_DATE)
                        .takes_value(true)
                        .short("t"),
                )
                .arg(clap::Arg::with_name(CLAP_EXPENDITURE_TYPE).required(true)),
        )
        .subcommand(
            clap::App::new("insert")
                .about("inserts an expenditure")
                .arg(
                    clap::Arg::with_name(CLAP_AMOUNT)
                        .takes_value(true)
                        .required(true)
                        .short("a"),
                )
                .arg(
                    clap::Arg::with_name(CLAP_DESCRIPTION)
                        .takes_value(true)
                        .required(true)
                        .short("d"),
                )
                .arg(
                    clap::Arg::with_name(CLAP_TARGET_DATE)
                        .takes_value(true)
                        .short("t"),
                )
                .arg(clap::Arg::with_name(CLAP_EXPENDITURE_TYPE).required(true)),
        )
        .get_matches();

    parse_clap_matches(matches)
}

fn expenditure_type_name_from_enum(type_: &ExpenditureType) -> &'static str {
    match type_ {
        ExpenditureType::Personal => return PEREXP,
        ExpenditureType::Essential => return ESSEXP,
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
