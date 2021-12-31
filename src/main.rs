use crate::command_parser::CommandMatchParser;
use crate::output_handler::{handle_error, handle_output};
use clap::{load_yaml, App};
use i18n_codegen::i18n;
use sap_bindings::net::object::Program;
use sap_bindings::net::{Responses, SAPClient};
use std::env::args_os;
use std::process::exit;
pub mod command_parser;
pub mod output_handler;

use sap_bindings::config::AppConfig;

#[tokio::main]
async fn main() {
    let cli_yaml = load_yaml!("cli.yaml");
    let mut app = App::from(cli_yaml);
    // Check if no argument was given because clap throws exit code 2
    if args_os().count() == 1 {
        app.print_help().expect("Could not print help");
        exit(0);
    }

    let matches = app.get_matches();

    let _conf = AppConfig::init();
    let mut app_conf = _conf.clone();
    // let mut prog = Program::new("ZPF_1511_2", None, None);
    let dest = app_conf.get_default_destination();
    // let mut config = prog.create();
    let mut parser = CommandMatchParser::new(&_conf, &dest.lang);
    let parsed = parser.parse(&matches);
    let mut config = parsed.0;
    let success_msg = parsed.1;

    let mut client: SAPClient;

    let update_session_file: bool;

    if let Some(session) = app_conf.get_session_from_sys(&dest.sys_id) {
        client = SAPClient::from_session(&dest, session);
        update_session_file = false;
    } else {
        client = SAPClient::new(&dest);
        update_session_file = true;
    }

    match config.send_with(&mut client).await {
        Ok(res) => {
            if res.get_value() == Responses::Default(String::new()) {
                handle_output(Responses::Default(success_msg))
            } else {
                handle_output(res.get_value())
            }
        }
        Err(e) => handle_error(&e),
    }
    // let success_msg = parser.get_success_smg();
    if update_session_file {
        // AppConfig::set_session_for_sys(&mut app_conf, "ITK", &client.get_session().unwrap());
        app_conf.set_session_for_sys("ITK", &client.get_session().unwrap());
    }
    app_conf.update_file();
}
