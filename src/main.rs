use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::process::exit;
use std::slice::SliceIndex;
use std::str::FromStr;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use chrono::{DateTime, Utc};
use clap::{load_yaml, App, ArgMatches, Parser};
use cli_table::{print_stdout, Cell, Style, Table};
use csv::WriterBuilder;
use ini::configparser::ini::Ini;
use quick_xml;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::{self, Client, Response};
use sap_adt_bindings::config::program_config::{Config, FreeStyleConfig, ProgramConfig};
use sap_adt_bindings::sap_client::{SAPClient, Session};
use serde::Deserialize;
use tokio;

#[derive(Parser)]
#[clap(version = "1.0", author = "Kevin K. <kbknapp@gmail.com>")]
struct Opts {
    /// Sets a custom config file. Could have been an Option<T> with no default too
    #[clap(short, long, default_value = "default.conf")]
    config: String,
    /// Some input. Because this isn't an Option<T> it's required to be used
    input: String,
    /// A level of verbosity, and can be used multiple times
    #[clap(short, long, parse(from_occurrences))]
    verbose: i32,
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Parser)]
enum SubCommand {
    #[clap(version = "1.3", author = "Someone E. <someone_else@other.com>")]
    Test(TableCommand),
}

/// A subcommand for controlling testing
#[derive(Parser)]
struct TableCommand {
    /// Print debug info
    #[clap(short)]
    name: String,
    rows: Option<u32>,
}

#[tokio::main]
async fn main() {
    let cli_yaml = load_yaml!("cli.yaml");

    let matches = App::from(cli_yaml).get_matches();

    let command_match_parser = CommandMatchParser::new();

    command_match_parser.parse(&matches).await;
}

struct CommandMatchParser {}

impl CommandMatchParser {
    fn new() -> CommandMatchParser {
        CommandMatchParser {}
    }

    async fn parse(self, matches: &ArgMatches) {
        match &matches.subcommand() {
            &Some(("table", table_matches)) => {
                let tab_name = table_matches.value_of("name").unwrap();
                let rows: Option<u32> = table_matches.value_of_t("rows").ok();
                let path = table_matches.value_of("out");

                fetch_table(format!("SELECT * FROM {0}", tab_name), rows, path).await;
            }
            &Some(("sql", sql_matches)) => {
                let sql_exp = sql_matches.value_of_t("sql_exp").unwrap();
                let rows: Option<u32> = sql_matches.value_of_t("rows").ok();

                fetch_table(sql_exp, rows, None).await;
            }
            &Some(("new", new_matches)) => match new_matches.subcommand() {
                Some(("prog", prog_matches)) => {
                    let prog_name = prog_matches.value_of_t("name").unwrap();
                    let package_name: String = prog_matches.value_of_t("package").unwrap();
                    let transport_request: String = prog_matches.value_of_t("transport").unwrap();

                    create_program(&prog_name, &package_name, &transport_request).await;
                }

                Some((_, _)) => {}
                None => {}
            },
            &Some((_, _)) => {}
            None => {}
        }
    }
}

#[derive(Debug, Deserialize)]
struct DataSet {
    data: Vec<Option<String>>,
}
#[derive(Debug, Deserialize)]
struct Columns {
    metadata: Metadata,
    #[serde(rename = "dataSet")]
    data_set: DataSet,
}

#[derive(Debug, Deserialize)]
struct Metadata {
    #[serde(rename = "dataPreview:name")]
    name: String,
    #[serde(rename = "dataPreview:type")]
    r#type: String,
    #[serde(rename = "dataPreview:description")]
    description: String,
    #[serde(rename = "dataPreview:keyAttribute")]
    keyAttribute: bool,
    #[serde(rename = "dataPreview:colType")]
    colType: String,
    #[serde(rename = "dataPreview:isKeyFigure")]
    isKeyFigure: bool,
}
#[derive(Debug, Deserialize)]
struct TableData {
    totalRows: u32,
    isHanaAnalyticalView: bool,
    executedQueryString: String,
    queryExecutionTime: String,
    columns: Vec<Columns>,
}
#[derive(Debug, Deserialize)]
struct XML {
    tableData: TableData,
}
async fn fetch_table(
    sql_exp: String,
    rows: Option<u32>,
    path: Option<&str>,
) -> core::result::Result<(), csv::Error> {
    let mut app_conf = AppConfig::init();
    let mut client: SAPClient;
    let host = "http://hamerpitk01.lej.it2-solutions.com:8000";
    let update_session_file: bool;

    if let Some(session) = app_conf.get_session_from_sys("ITK") {
        client = SAPClient::from_session(host, session);
        update_session_file = false;
    } else {
        client = SAPClient::new(&String::from(host));
        update_session_file = true;
    }

    let res = client.send(&FreeStyleConfig::new(sql_exp, rows)).await;

    let xml = res.text().await.unwrap();
    let table_data: TableData = quick_xml::de::from_str(&xml).unwrap();

    let mut abap_table = ABAPTable::new(table_data);

    abap_table.build();

    if path.is_some() {
        let mut writer = WriterBuilder::new()
            .delimiter(b';')
            .from_path(path.unwrap())
            .unwrap();

        println!("{0}", path.unwrap());

        let headers = abap_table.get_headers();
        let borrowed_headers: Vec<&String> = headers.iter().map(|s| s).collect();
        writer.write_record(borrowed_headers)?;
        let data = abap_table.get_data();
        let mut iter = data.iter();

        while let Some(v) = iter.next() {
            let new_v: Vec<&String> = v.iter().map(|s| s).collect();

            writer.write_record(&new_v)?;

            writer.flush()?;
        }
    }
    abap_table.display();
    if update_session_file {
        app_conf.set_session_for_sys("ITK", &client.get_session().unwrap());
        app_conf.update_file();
    }
    Ok(())
}

struct ABAPTable {
    headers: Vec<String>,
    data: Vec<Vec<String>>,
    table_data: TableData,
}

impl ABAPTable {
    pub fn new(table_data: TableData) -> ABAPTable {
        ABAPTable {
            headers: vec![],
            data: vec![],
            table_data,
        }
    }
    pub fn build(&mut self) {
        &self.extract_headers();

        &self.extract_data();
    }
    fn get_headers(&self) -> Vec<String> {
        self.headers.to_owned()
    }
    fn extract_headers(&mut self) {
        self.headers = self
            .table_data
            .columns
            .iter()
            .map(|column: &Columns| String::from(&column.metadata.name))
            // .map(|t: &String| t.cell().bold(true))
            .collect();
    }

    fn extract_data(&mut self) {
        let len = 0..self.table_data.columns[0].data_set.data.len();

        let mut i: usize = 0;
        let mut data: Vec<Vec<String>> = vec![];

        loop {
            let mut data_vec: Vec<String> = vec![];

            for col in self.table_data.columns.iter() {
                let data = &col.data_set.data[i];

                data_vec.push(Self::option_to_string(data));
            }

            if i == len.end - 1 {
                break;
            } else {
                i = i + 1;
                data.push(data_vec);
            }
        }
        self.data = data;
    }

    fn option_to_string(option: &Option<String>) -> String {
        match option {
            Some(val) => val.to_string(),
            None => String::from(""),
        }
    }

    pub fn get_data(&self) -> Vec<Vec<String>> {
        self.data.to_owned()
    }

    pub fn display(self) {
        print_stdout(
            self.data
                .table()
                .title(self.headers.iter().map(|t: &String| t.cell().bold(true))),
        );
    }
}

struct AppConfig {
    config: Ini,
}

impl AppConfig {
    pub fn init() -> Self {
        let mut conf = AppConfig { config: Ini::new() };
        if conf.config.load("sapClient.ini").is_err() {
            std::fs::File::create("sapClient.ini");
        }

        conf
    }
    pub fn get_session_from_sys(&mut self, sys_id: &str) -> Option<Session> {
        let section = format!("session_{0}", sys_id);
        let expires_string = self.config.get(&section, "expires")?;
        let expires: DateTime<Utc> = DateTime::from_str(&expires_string).ok()?;

        if expires - Utc::now() <= chrono::Duration::zero() {
            // Session is over
            return None;
        }

        Some(Session {
            csrf_token: self.config.get(&section, "csrf_token")?,
            session_cookie: self.config.get(&section, "session_cookie")?,
        })
    }
    pub fn set_session_for_sys(&mut self, sys_id: &str, session: &Session) {
        let section = format!("session_{0}", sys_id);
        self.config
            .set(&section, "csrf_token", Some(session.csrf_token.clone()));
        self.config.set(
            &section,
            "session_cookie",
            Some(session.session_cookie.clone()),
        );
        self.config.set(
            &section,
            "expires",
            Some(
                Utc::now()
                    .checked_add_signed(chrono::Duration::minutes(15))
                    .unwrap()
                    .to_string(),
            ),
        );
    }

    pub fn update_file(&mut self) {
        self.config.write("sapClient.ini");
    }
}

async fn create_program(prog_name: &String, package_nam: &str, transport_request: &str) {
    let mut client = SAPClient::new(&String::from(
        "http://hamerpitk01.lej.it2-solutions.com:8000",
    ));

    let res = client
        .send(&ProgramConfig::new(
            &prog_name,
            package_nam,
            transport_request,
        ))
        .await;

    let status = res.status();
    let text = res.text().await.unwrap();
    println!("{}", status);
    println!("{}", text);

    if status.is_success() {
        println!("Programm wurde erstellt");
    }
}
