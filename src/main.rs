use crate::command_parser::CommandMatchParser;
use crate::output_handler::{handle_error, handle_output};
use app_config::AppConfig;
use clap::{load_yaml, App};
use sap_adt_bindings::net::SAPClient;
pub mod app_config;
pub mod command_parser;
pub mod output_handler;

#[tokio::main]
async fn main() {
    let cli_yaml = load_yaml!("cli.yaml");

    let matches = App::from(cli_yaml).get_matches();

    let mut config = CommandMatchParser::parse(&matches);
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

    match config.send_with(&mut client).await {
        Ok(()) => handle_output(config.get_response().unwrap()),
        Err(e) => handle_error(e),
    }

    if update_session_file {
        app_conf.set_session_for_sys("ITK", &client.get_session().unwrap());
        app_conf.update_file();
    }
}
// macro_rules! cast {
//     ($target: expr, $pat: path) => {{
//         if let $pat(a) = $target {
//             // #1
//             a
//         } else {
//             panic!("mismatch variant when cast to {}", stringify!($pat)); // #2
//         }
//     }};
// }
// AppConfig::read_destination_file();
// struct CommandMatchParser {}

// // struct TableCommand {
// //     config: FreeStyleConfig,
// // }
// // impl TableCommand {
// //     fn new(matches: ArgMatches) -> Self {
// //         TableCommand {
// //             config: FreeStyleConfig::new(
// //                 format!("SELECT * FROM {}", matches.value_of("name").unwrap()),
// //                 matches.value_of_t("rows").ok(),
// //             ),
// //         }
// //     }
// //     fn get_config(&self) -> FreeStyleConfig {
// //         self.config
// //     }
// // }
// trait CommandParser {
//     fn parse<'a>(args: &'a ArgMatches) -> &'a dyn Config;
// }
// struct TableCommandParser {}
// struct SqlCommandParser {}
// impl CommandParser for TableCommandParser {
//     fn parse<'a>(args: &'a ArgMatches) -> &'a dyn Config {
//         let tab_name = args.value_of("name").unwrap();
//         let rows: Option<u32> = args.value_of_t("rows").ok();
//         // let path = args.value_of("out");
//         &FreeStyleConfig::new(format!("SELECT * FROM {0}", tab_name), rows)
//     }
// }
// impl CommandParser for SqlCommandParser {
//     fn parse<'a>(args: &'a ArgMatches) -> &'a dyn Config {
//         &FreeStyleConfig::new(
//             args.value_of_t("sql_exp").unwrap(),
//             args.value_of_t("rows").ok(),
//         )
//     }
// }
// impl CommandMatchParser {
//     fn new() -> CommandMatchParser {
//         CommandMatchParser {}
//     }

//     async fn parse(self, matches: &ArgMatches) {
//         // let ma
//         match &matches.subcommand() {
//             &Some(("table", table_matches)) => {
//                 let tab_name = table_matches.value_of("name").unwrap();
//                 let rows: Option<u32> = table_matches.value_of_t("rows").ok();
//                 let path = table_matches.value_of("out");

//                 fetch_table(format!("SELECT * FROM {0}", tab_name), rows, path).await;
//             }
//             &Some(("sql", sql_matches)) => {
//                 let sql_exp = sql_matches.value_of_t("sql_exp").unwrap();
//                 let rows: Option<u32> = sql_matches.value_of_t("rows").ok();

//                 fetch_table(sql_exp, rows, None).await;
//             }
//             &Some(("new", new_matches)) => match new_matches.subcommand() {
//                 Some(("prog", prog_matches)) => {
//                     let prog_name = prog_matches.value_of_t("name").unwrap();
//                     let package_name: String = prog_matches.value_of_t("package").unwrap();
//                     let transport_request: String = prog_matches.value_of_t("transport").unwrap();

//                     create_program(&prog_name, &package_name, &transport_request).await;
//                 }
//                 Some(("class", class_matches)) => {
//                     let class_name: String = class_matches.value_of_t("name").unwrap();
//                     let package_name: String = class_matches.value_of_t("package").unwrap();
//                     let transport_request: String = class_matches.value_of_t("transport").unwrap();

//                     create_class(&class_name, &package_name, &transport_request).await;
//                 }

//                 Some((_, _)) => {}
//                 None => {}
//             },
//             &Some(("copy", copy_matches)) => {
//                 let source_name: String = copy_matches.value_of_t("source").unwrap();
//                 let prog_name: String = copy_matches.value_of_t("name").unwrap();
//                 let package_name: String = copy_matches.value_of_t("package").unwrap();
//                 let transport_request: String = copy_matches.value_of_t("transport").unwrap();

//                 copy_program(&source_name, &prog_name, &transport_request, &package_name).await;
//             }
//             &Some(("transport", transport_matches)) => {}
//             &Some(("delete", delete_matches)) => {
//                 let prog_name: String = delete_matches.value_of_t("name").unwrap();

//                 delete_program(&prog_name).await;
//             }
//             &Some((_, _)) => {}
//             None => {}
//         }
//     }
// }
// async fn copy_program_to_sys(
//     source_prog_name: &str,
//     prog_name: &str,
//     transport_request: &str,
//     package_name: &str,
//     sys_name: &str,
// ) {
//     let mut app_conf = AppConfig::init();
//     let mut client: SAPClient;
//     let host = "http://hamerpitk01.lej.it2-solutions.com:8000";
//     let update_session_file: bool;

//     if let Some(session) = app_conf.get_session_from_sys("ITK") {
//         client = SAPClient::from_session(host, session);
//         update_session_file = false;
//     } else {
//         client = SAPClient::new(&String::from(host));
//         update_session_file = true;
//     }

//     let destination = app_conf.get_destination_from_sys("ITK").unwrap();
//     let to_sys = app_conf.get_destination_from_sys(sys_name).unwrap();

//     // create_program(prog_name, package_name, transport_request)
// }
// async fn copy_program(
//     source_prog_name: &str,
//     prog_name: &str,
//     transport_request: &str,
//     package_name: &str,
// ) {
//     let mut app_conf = AppConfig::init();
//     let mut client: SAPClient;
//     let host = "http://hamerpitk01.lej.it2-solutions.com:8000";
//     let update_session_file: bool;

//     if let Some(session) = app_conf.get_session_from_sys("ITK") {
//         client = SAPClient::from_session(host, session);
//         update_session_file = false;
//     } else {
//         client = SAPClient::new(&String::from(host));
//         update_session_file = true;
//     }
//     // client.set_stateful(true);
//     // let lock_handle_res = client.send(&LockHandle::new(prog_name)).await;

//     // let xml = lock_handle_res.text().await.unwrap();
//     // // println!("{:?}", &xml);
//     // let lock_handle: LockHandleResponse = quick_xml::de::from_str(&xml).unwrap();

//     let res = client
//         .send(&ProgramConfig::copy(
//             &prog_name,
//             package_name,
//             &source_prog_name,
//             transport_request,
//             // &lock_handle.values.DATA.LOCK_HANDLE,
//         ))
//         .await;

//     let status = res.status();
//     let text = res.text().await.unwrap();
//     println!("{}", status);
//     println!("{}", text);

//     if status.is_success() {
//         println!("Programm wurde kopiert");
//     }

//     if update_session_file {
//         app_conf.set_session_for_sys("ITK", &client.get_session().unwrap());
//         app_conf.update_file();
//     }
// }

// async fn fetch_table(
//     sql_exp: String,
//     rows: Option<u32>,
//     path: Option<&str>,
// ) -> core::result::Result<(), csv::Error> {
//     let mut app_conf = AppConfig::init();
//     let mut client: SAPClient;
//     let host = "http://hamerpitk01.lej.it2-solutions.com:8000";
//     let update_session_file: bool;

//     if let Some(session) = app_conf.get_session_from_sys("ITK") {
//         client = SAPClient::from_session(host, session);
//         update_session_file = false;
//     } else {
//         client = SAPClient::new(&String::from(host));
//         update_session_file = true;
//     }

//     let res = client.send(&FreeStyleConfig::new(sql_exp, rows)).await;

//     let xml = res.text().await.unwrap();
//     let table_data: TableData = quick_xml::de::from_str(&xml).unwrap();

//     let mut abap_table = ABAPTable::new(table_data);

//     abap_table.build();

//     if path.is_some() {
//         let mut writer = WriterBuilder::new()
//             .delimiter(b';')
//             .from_path(path.unwrap())
//             .unwrap();

//         println!("{0}", path.unwrap());

//         let headers = abap_table.get_headers();
//         let borrowed_headers: Vec<&String> = headers.iter().map(|s| s).collect();
//         writer.write_record(borrowed_headers)?;
//         let data = abap_table.get_data();
//         let mut iter = data.iter();

//         while let Some(v) = iter.next() {
//             let new_v: Vec<&String> = v.iter().map(|s| s).collect();

//             writer.write_record(&new_v)?;

//             writer.flush()?;
//         }
//     }
//     abap_table.display();
//     if update_session_file {
//         app_conf.set_session_for_sys("ITK", &client.get_session().unwrap());
//         app_conf.update_file();
//     }
//     Ok(())
// }
// #[derive(Debug, Deserialize)]
// #[serde(rename = "DATA")]
// struct LockHandleData {
//     LOCK_HANDLE: String,
//     CORRNR: String,
//     CORRUSER: String,
//     CORRTEXT: String,
// }

// #[derive(Debug, Deserialize)]
// struct LockHandleValues {
//     DATA: LockHandleData,
// }
// #[derive(Debug, Deserialize)]
// #[serde(rename = "asx:abap")]
// struct LockHandleResponse {
//     // #[serde(rename = "asx:values")]
//     values: LockHandleValues,
// }

// async fn delete_program(prog_name: &str) {
//     let mut app_conf = AppConfig::init();
//     let mut client: SAPClient;
//     let host = "http://hamerpitk01.lej.it2-solutions.com:8000";
//     let update_session_file: bool;

//     if let Some(session) = app_conf.get_session_from_sys("ITK") {
//         client = SAPClient::from_session(host, session);
//         update_session_file = false;
//     } else {
//         client = SAPClient::new(&String::from(host));
//         update_session_file = true;
//     }
//     client.set_stateful(true);
//     let lock_handle_res = client.send(&LockHandle::new(prog_name)).await;

//     let xml = lock_handle_res.text().await.unwrap();
//     // println!("{:?}", &xml);
//     let lock_handle: LockHandleResponse = quick_xml::de::from_str(&xml).unwrap();
//     println!("{:?}", &lock_handle);
//     let res = client
//         .delete(&ProgramConfig::delete(
//             &prog_name,
//             &lock_handle.values.DATA.LOCK_HANDLE,
//             &lock_handle.values.DATA.CORRNR,
//         ))
//         .await;

//     let status = res.status();
//     let text = res.text().await.unwrap();
//     println!("{}", status);
//     println!("{}", text);

//     if status.is_success() {
//         println!("Programm wurde gelöscht");
//     }

//     if update_session_file {
//         app_conf.set_session_for_sys("ITK", &client.get_session().unwrap());
//         app_conf.update_file();
//     }
// }

// async fn create_class(class_name: &String, package_nam: &str, transport_request: &str) {
//     let mut app_conf = AppConfig::init();
//     let mut client: SAPClient;
//     let host = "http://hamerpitk01.lej.it2-solutions.com:8000";
//     let update_session_file: bool;

//     if let Some(session) = app_conf.get_session_from_sys("ITK") {
//         client = SAPClient::from_session(host, session);
//         update_session_file = false;
//     } else {
//         client = SAPClient::new(&String::from(host));
//         update_session_file = true;
//     }
//     let res = client
//         .send(&ClassConfig::new(
//             &class_name,
//             package_nam,
//             transport_request,
//         ))
//         .await;

//     let status = res.status();
//     let text = res.text().await.unwrap();
//     println!("{}", status);
//     println!("{}", text);

//     if status.is_success() {
//         println!("Klasse wurde erstellt");
//     }

//     if update_session_file {
//         app_conf.set_session_for_sys("ITK", &client.get_session().unwrap());
//         app_conf.update_file();
//     }
// }
// async fn create_program(prog_name: &String, package_nam: &str, transport_request: &str) {
//     let mut app_conf = AppConfig::init();
//     let mut client: SAPClient;
//     let host = "http://hamerpitk01.lej.it2-solutions.com:8000";
//     let update_session_file: bool;

//     if let Some(session) = app_conf.get_session_from_sys("ITK") {
//         client = SAPClient::from_session(host, session);
//         update_session_file = false;
//     } else {
//         client = SAPClient::new(&String::from(host));
//         update_session_file = true;
//     }

//     let res = client
//         .send(&ProgramConfig::new(
//             &prog_name,
//             package_nam,
//             transport_request,
//         ))
//         .await;

//     let status = res.status();
//     let text = res.text().await.unwrap();
//     println!("{}", status);
//     println!("{}", text);

//     if status.is_success() {
//         println!("Programm wurde erstellt");
//     }

//     if update_session_file {
//         app_conf.set_session_for_sys("ITK", &client.get_session().unwrap());
//         app_conf.update_file();
//     }
// }
