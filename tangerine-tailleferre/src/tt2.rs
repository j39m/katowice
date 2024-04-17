const DB_PATH: &'static str = "./expenditures.db";

const TABLE_NAME: &'static str = "expenditures";
const SCHEMA_AMOUNT: &'static str = "amount";
const SCHEMA_CURRENCY: &'static str = "currency";
const SCHEMA_DESCRIPTION: &'static str = "description";
const SCHEMA_TYPE: &'static str = "type";
const SCHEMA_TARGET_DATE: &'static str = "date";

#[derive(Debug)]
#[repr(u8)]
pub enum Currency {
    USD = 0,
    JPY = 1,
}

#[derive(Debug)]
#[repr(u8)]
pub enum Type {
    Essential = 0,
    Personal = 1,
    GeneralInformational = 2,
    MiraclePtrBonus = 3,
}

#[derive(Debug)]
pub struct SqlCoreValues {
    target_date: chrono::NaiveDate,
    currency: Currency,
    etype: Type,
}

#[derive(Debug)]
pub struct SqlInsertValues {
    core: SqlCoreValues,
    amount: f64,
    description: String,
}

#[derive(Debug)]
pub enum SqlCommand {
    Show(SqlCoreValues),
    Insert(SqlInsertValues),
}

mod from_clap {
    use clap::Parser;

    #[derive(clap::Parser)]
    #[command(name = "tt2")]
    #[command(version = "2.0.0")]
    #[command(about = "manipulates expenditures")]
    struct Cli {
        #[command(subcommand)]
        subcommand: Subcommand,
    }

    #[derive(clap::Subcommand)]
    enum Subcommand {
        Show(ShowArgs),
        Insert(InsertArgs),
    }

    #[derive(clap::Args)]
    struct ShowArgs {
        #[arg(short, help = "(optional) date or past-facing offset")]
        target_date: Option<String>,
        #[command(flatten)]
        currency: Currency,
        #[command(flatten)]
        etype: Type,
    }

    #[derive(clap::Args)]
    struct InsertArgs {
        #[arg(short, help = "(optional) date or past-facing offset")]
        target_date: Option<String>,
        #[command(flatten)]
        currency: Currency,
        #[command(flatten)]
        etype: Type,
        #[arg(short, help = super::SCHEMA_AMOUNT)]
        amount: f64,
        #[arg(short, help = super::SCHEMA_DESCRIPTION)]
        description: String,
    }

    #[derive(clap::Args)]
    #[group(required = true, multiple = false)]
    struct Currency {
        #[arg(short, help = "currency = USD")]
        usd: bool,
        #[arg(short, help = "currency = JPY")]
        jpy: bool,
    }

    #[derive(clap::Args)]
    #[group(required = true, multiple = false)]
    struct Type {
        #[arg(short, help = "type = essential")]
        essential: bool,
        #[arg(short, help = "type = personal")]
        personal: bool,
        #[arg(short, help = "type = general informational")]
        general_informational: bool,
        #[arg(short, help = "type = 2024Q1 bonus")]
        miracleptr_bonus: bool,
    }

    fn currency(from_clap: Currency) -> super::Currency {
        if from_clap.usd {
            return super::Currency::USD;
        } else if from_clap.jpy {
            return super::Currency::JPY;
        }
        panic!("BUG: currency");
    }

    fn expenditure_type(from_clap: Type) -> super::Type {
        if from_clap.essential {
            return super::Type::Essential;
        } else if from_clap.personal {
            return super::Type::Personal;
        } else if from_clap.general_informational {
            return super::Type::GeneralInformational;
        } else if from_clap.miracleptr_bonus {
            return super::Type::MiraclePtrBonus;
        }
        panic!("BUG: type")
    }

    fn target_date(from_clap: Option<String>) -> Option<chrono::NaiveDate> {
        if let Some(ymd) = from_clap {
            if let Ok(date) = chrono::NaiveDate::parse_from_str(ymd.as_str(), "%Y-%m-%d") {
                return Some(date);
            }
            if let Ok(date_delta) = ymd.parse::<i64>() {
                return Some(
                    (chrono::Local::now() - chrono::TimeDelta::try_days(date_delta).unwrap())
                        .date_naive(),
                );
            }
            panic!("bad target date: ``{}''", ymd);
        }
        None
    }

    fn build_show_options(args: ShowArgs) -> super::SqlCoreValues {
        let target_date = match target_date(args.target_date) {
            Some(date) => date,
            // Aribtrary choice: peeks back 6 months.
            None => (chrono::Local::now() - chrono::TimeDelta::try_days(183).unwrap()).date_naive(),
        };

        super::SqlCoreValues {
            target_date: target_date,
            currency: currency(args.currency),
            etype: expenditure_type(args.etype),
        }
    }

    fn build_insert_options(args: InsertArgs) -> super::SqlInsertValues {
        let target_date = match target_date(args.target_date) {
            Some(date) => date,
            None => chrono::Local::now().date_naive(),
        };

        super::SqlInsertValues {
            core: super::SqlCoreValues {
                target_date: target_date,
                currency: currency(args.currency),
                etype: expenditure_type(args.etype),
            },
            amount: args.amount,
            description: args.description,
        }
    }

    pub fn parse() -> super::SqlCommand {
        let cli = Cli::parse();
        match cli.subcommand {
            Subcommand::Show(args) => super::SqlCommand::Show(build_show_options(args)),
            Subcommand::Insert(args) => super::SqlCommand::Insert(build_insert_options(args)),
        }
    }
} // mod from_clap

//options.target_date.format("%Y-%m-%d").to_string()

fn show(connection: sqlite::Connection, values: SqlCoreValues) {
    let prepared = format!(
        "select * from {} where {} >= ? and {} = ? and {} = ?",
        TABLE_NAME, SCHEMA_TARGET_DATE, SCHEMA_CURRENCY, SCHEMA_TYPE
    );
    let mut statement = connection.prepare(prepared).unwrap();
    statement
        .bind((
            1,
            values.target_date.format("%Y-%m-%d").to_string().as_str(),
        ))
        .unwrap();
    statement.bind((2, values.currency as i64)).unwrap();
    statement.bind((3, values.etype as i64)).unwrap();

    let mut sum: f64 = 0.0;
    while let Ok(sqlite::State::Row) = statement.next() {
        let date = statement.read::<String, _>(SCHEMA_TARGET_DATE).unwrap();
        let description = statement.read::<String, _>(SCHEMA_DESCRIPTION).unwrap();
        let amount = statement.read::<f64, _>(SCHEMA_AMOUNT).unwrap();
        sum += amount;
        println!("{}|{}|{}", date, description, amount);
    }
    println!("{}", sum);
}

fn insert(connection: sqlite::Connection, values: SqlInsertValues) {
    let statement = format!(
        "insert into {} values('{}', '{}', {}, {}, {})",
        TABLE_NAME,
        values.core.target_date,
        values.description,
        values.amount,
        values.core.currency as u8,
        values.core.etype as u8
    );
    connection.execute(statement).unwrap();
}

fn main() {
    let command = from_clap::parse();
    eprintln!("{:#?}", command);

    let connection = sqlite::open(DB_PATH).unwrap();
    connection.execute("pragma foreign_keys = on").unwrap();
    match command {
        SqlCommand::Show(values) => show(connection, values),
        SqlCommand::Insert(values) => insert(connection, values),
    }
}
