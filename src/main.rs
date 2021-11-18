use crate::command_parser::CommandMatchParser;
use crate::output_handler::{handle_error, handle_output};

use clap::{load_yaml, App};
use sap_adt_bindings::net::SAPClient;

pub mod command_parser;
pub mod output_handler;

use sap_adt_bindings::app_config::AppConfig;

#[tokio::main]
async fn main() {
    let cli_yaml = load_yaml!("cli.yaml");

    let matches = App::from(cli_yaml).get_matches();

    let mut config = CommandMatchParser::parse(&matches);

    let mut app_conf = AppConfig::init();
    let mut client: SAPClient;

    let dest = app_conf.get_default_destination();
    println!("{:?}", dest);
    let update_session_file: bool;

    if let Some(session) = app_conf.get_session_from_sys(&dest.sys_id) {
        client = SAPClient::from_session(&dest, session);
        update_session_file = false;
    } else {
        client = SAPClient::new(&dest);
        update_session_file = true;
    }

    match config.send_with(&mut client).await {
        Ok(()) => handle_output(config.get_response().unwrap()),
        Err(e) => handle_error(e),
    }

    if update_session_file {
        app_conf.set_session_for_sys("ITK", &client.get_session().unwrap());
    }
    app_conf.update_file();
}
