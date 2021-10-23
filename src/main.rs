extern crate clap;

use clap::{load_yaml, App, AppSettings, ArgMatches, Parser};
use std::fmt::Result;
use std::future::Future;

use clap::{value_t, Arg, Values};
use cli_table::{format::Justify, print_stdout, Cell, CellStruct, Row, Style, Table};
use minidom;
use quick_xml;
use reqwest::header::HeaderValue;
use reqwest::Error;
use reqwest::{self, Client, Response};
use sap_adt_bindings::config::program_config::{Config, FreeStyleConfig, ProgramConfig};
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

                fetch_table(format!("SELECT * FROM {0}", tab_name), rows).await;
            }
            &Some(("sql", sql_matches)) => {
                let sql_exp = sql_matches.value_of_t("sql_exp").unwrap();
                let rows: Option<u32> = sql_matches.value_of_t("rows").ok();

                fetch_table(sql_exp, rows).await;
           
            }
            &Some(("new", new_matches)) => match new_matches.subcommand() {
                Some(("prog", prog_matches)) => {}
                Some((_, _)) => {}
                None => {}
            }
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
async fn fetch_table(sql_exp: String, rows: Option<u32>) {

    let mut client = SAPClient::new(&String::from(
        "http://hamerpitk01.lej.it2-solutions.com:8000",
    ));
    let res = client.send(&FreeStyleConfig::new(sql_exp, rows)).await;

    let xml = res.text().await.unwrap();
    let tab: TableData = quick_xml::de::from_str(&xml).unwrap();

    let mut data_fields: Vec<Vec<String>> = vec![];
    let mut index: usize = 0;

    let titles_strings: Vec<&String> = tab
        .columns
        .iter()
        .map(|column: &Columns| &column.metadata.name)
        .collect();
    let titles: Vec<CellStruct> = titles_strings
        .iter()
        .map(|t: &&String| t.cell().bold(true))
        .collect();
    let len = 0..tab.columns[0].data_set.data.len();
    loop {
        let mut data_vec: Vec<String> = vec![];

        for col in tab.columns.iter() {
            let data = &col.data_set.data[index];

            data_vec.push(match &data {
                Some(val) => String::from(val),
                None => String::from(""),
            });
        }
        data_fields.push(data_vec);
        if index == len.end - 1 {
            break;
        } else {
            index = index + 1;
        }
    }

    let table = data_fields.table().title(titles);
    print_stdout(table);
}
struct SAPClient {
    client: Client,

    csrf_token: Option<HeaderValue>,
    host: String,
}

impl SAPClient {
    fn new(host: &String) -> SAPClient {
        SAPClient {
            client: reqwest::Client::builder()
                .cookie_store(true)
                .build()
                .unwrap(),
            host: host.to_string(),
            csrf_token: None,
        }
    }

    async fn fetch_csrf_token(&mut self) {
        let res = &self
            .client
            .get(format!(
                "{}{}",
                &self.host, "/sap/bc/adt/programs?sap-client=300"
            ))
            .basic_auth("pfrank", Some("Start123!"))
            .header("x-csrf-token", "Fetch")
            .send()
            .await
            .unwrap();

        self.csrf_token = Some(res.headers().get("x-csrf-token").unwrap().clone());
    }

    async fn send(&mut self, config: &impl Config) -> Response {
        if self.csrf_token.is_none() {
            self.fetch_csrf_token().await;
        }

        let url = format!("{0}{1}", &self.host, &config.get_path());

        self.client
            .post(&url)
            .basic_auth("pfrank", Some("Start123!"))
            .header("x-csrf-token", self.csrf_token.as_ref().unwrap())
            .body(String::from(config.get_body()))
            .send()
            .await
            .unwrap()
    }

    async fn get<T: Config>(&mut self, config: T) -> Response {
        if self.csrf_token.is_none() {
            self.fetch_csrf_token().await;
        }

        let url = format!("{0}{1}", &self.host, &config.get_path());

        self.client
            .get(&url)
            .basic_auth("pfrank", Some("Start123!"))
            .header("x-csrf-token", self.csrf_token.as_ref().unwrap())
            .body(String::from(config.get_body()))
            .send()
            .await
            .unwrap()
    }
}

async fn create_program(prog_name: &String) {
    let mut client = SAPClient::new(&String::from(
        "http://hamerpitk01.lej.it2-solutions.com:8000",
    ));

    let res = client.send(&ProgramConfig::new(&prog_name)).await;

    let status = res.status();
    let text = res.text().await.unwrap();
    println!("{}", status);
    println!("{}", text);

    if status.is_success() {
        println!("Programm wurde erstellt");
    }
}
