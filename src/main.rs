use crate::command_parser::CommandMatchParser;
use crate::output_handler::{handle_error, handle_output};
use clap::{load_yaml, App, SubCommand};
use sap_bindings::net::object::Program;
use sap_bindings::net::{Responses, SAPClient};
use std::env::args_os;
use std::process::exit;
pub mod command_parser;
pub mod output_handler;
use sap_bindings::net::behavior::Create;

use sap_bindings::config::AppConfig;

#[tokio::main]
async fn main() {
    let _conf = AppConfig::init();

    let cli_yaml = load_yaml!("cli.yaml");
    let mut app = App::from(cli_yaml);

    // Check if no argument was given because clap throws exit code 2
    if args_os().count() == 1 {
        app.print_help().expect("Could not print help");
        exit(0);
    }

    let matches = app.get_matches();

    let mut app_conf = _conf.clone();

    let dest = app_conf.get_default_destination();
    let mut parser = CommandMatchParser::new(&_conf, &dest.lang);

    let mut client: SAPClient;

    let update_session_file: bool;

    if let Some(session) = app_conf.get_session_from_sys(&dest.sys_id) {
        client = SAPClient::from_session(&dest, session);
        update_session_file = false;
    } else {
        client = SAPClient::new(&dest);
        update_session_file = true;
    }

    if parser.is_check_command(&matches) {
        for dest in app_conf.get_all_destinations().iter() {
            match client.test_destination().await {
                Ok(()) => println!("{}-{}: [92m✓[0m", dest.sys_id.to_uppercase(), dest.mandt),
                Err(e) => println!("{}-{}: [31m✗[0m", dest.sys_id.to_uppercase(), dest.mandt),
            }
        }
    }
    if parser.is_dest_command(&matches) {
        AppConfig::open_destination_file()
    }
    //  let dest: Destination = {...}

    let parsed = parser.parse(&matches);
    let mut config = parsed.0;
    let success_msg = parsed.1;

    println!("{:?}", matches.subcommand());
    // match matches.subcommand() {
    //     Some(("check", matches)) => match client.test_destination().await {
    //         Ok(()) => println!("Check for default destination was successful"),
    //         Err(e) => println!("xxxxxxxxxxxxxxx"),
    //     },
    //     _ => println!("yyyyyyyyyyyyyyy"),
    // }
    // if Some(matches.subcommand()) == ("", _) {
    //     match client.test_destination().await {
    //         Some(()) => println("Check for default destination was successful"),
    //         Err(e) => println!("{}", e),
    //     }
    // }

    match config.send_with(&mut client).await {
        Ok(res) => match success_msg {
            Some(msg) => handle_output(Responses::Default(msg)),
            None => handle_output(res.get_value()),
        },
        Err(e) => handle_error(&e),
    }

    if update_session_file {
        app_conf.set_session_for_sys("ITK", &client.get_session().unwrap());
    }
    app_conf.update_file();
}
