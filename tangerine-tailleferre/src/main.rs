use std::os::unix::process::CommandExt;
use std::process::Command;

const SQLITE3: &'static str = "/usr/bin/sqlite3";
const DB_PATH: &'static str = "/home/kalvin/Documents/personal/expenditures.db";

const PEREXP: &'static str = "perexp";
const ESSEXP: &'static str = "essexp";

const SELECT_FMT: &'static str = r#"select {} from {} where date >= date("{}");"#;
const INSERT_FMT: &'static str = r#"insert into {} values(date("{}"), "{}", {});"#;

#[derive(Debug)]
struct SqlOptions {
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

fn expenditure_target_date_from_clap(matches: &clap::ArgMatches) -> chrono::Date<chrono::Utc> {
    panic!("XXX j39m")
}

fn expenditure_amount_from_clap(matches: &clap::ArgMatches) -> f64 {
    panic!("XXX j39m")
}

fn expenditure_description_from_clap(matches: &clap::ArgMatches) -> String {
    panic!("XXX j39m")
}

fn build_show_options(matches: &clap::ArgMatches) -> SqlOptions {
    SqlOptions {
        target_date: expenditure_target_date_from_clap(matches),
        amount: Some(expenditure_amount_from_clap(matches)),
        description: Some(expenditure_description_from_clap(matches)),
    }
}

fn build_insert_options(matches: &clap::ArgMatches) -> SqlOptions {
    panic!("XXX j39m");
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
                    clap::Arg::with_name("target-date")
                        .takes_value(true)
                        .short("t"),
                )
                .arg(clap::Arg::with_name("expenditure-type").required(true)),
        )
        .subcommand(
            clap::App::new("insert")
                .about("inserts an expenditure")
                .arg(
                    clap::Arg::with_name("amount")
                        .takes_value(true)
                        .required(true)
                        .short("a"),
                )
                .arg(
                    clap::Arg::with_name("description")
                        .takes_value(true)
                        .required(true)
                        .short("d"),
                )
                .arg(
                    clap::Arg::with_name("target-date")
                        .takes_value(true)
                        .short("t"),
                )
                .arg(clap::Arg::with_name("expenditure-type").required(true)),
        )
        .get_matches();

    parse_clap_matches(matches)
}

fn build_show_command(options: SqlOptions) -> String {
    String::new() // XXX j39m
}

fn build_insert_command(options: SqlOptions) -> String {
    String::new() // XXX j39m
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
