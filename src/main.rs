use crate::command_parser::CommandMatchParser;
use crate::output_handler::{handle_error, handle_output};
use clap::{load_yaml, App, AppSettings, ArgMatches, SubCommand};
use sap_bindings::net::object::Program;
use sap_bindings::net::{Responses, SAPClient};
use std::env::args_os;
use std::process::exit;
pub mod command_parser;
pub mod output_handler;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use sap_bindings::config::AppConfig;
use sap_bindings::net::behavior::Create;

#[tokio::main]
async fn main() {
    // let _conf = AppConfig::init();

    // `()` can be used when no completer is required
    let mut rl = Editor::<()>::new();
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }
    let cli_yaml = load_yaml!("cli.yaml");
    let mut app = App::from(cli_yaml);

    if args_os().count() == 1 {
        app.print_help().expect("Could not print help");
        // exit(0);
    } else {
        send_from_matches(app.get_matches()).await;
    }

    loop {
        let readline = rl.readline("quick_sap>> ");
        let line: String = match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                // println!("Line: {}", line);
                line
            }
            Err(ReadlineError::Interrupted) => {
                // println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                // println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        };
        let app = App::from(cli_yaml).setting(AppSettings::NoBinaryName);
        rl.save_history("history.txt").unwrap();

        let matches = app.get_matches_from(line.split_whitespace());
        send_from_matches(matches).await;
    }

    async fn send_from_matches(matches: ArgMatches) {
        let _conf = AppConfig::init();
        let mut app_conf = _conf.clone();

        let mut parser = CommandMatchParser::new(&_conf, "EN");
        if parser.is_dest_command(&matches) {
            AppConfig::open_destination_file();
            exit(0);
        }
        if parser.is_settings_command(&matches) {
            if let Some(sys_id) = matches
                .subcommand_matches("settings")
                .unwrap()
                .value_of("default_sys")
            {
                app_conf.set_default_sys(sys_id);
                app_conf.update_file();
                println!("The default system is now {}", sys_id);
                exit(0);
            }
        }
        let dest = app_conf.get_default_destination();

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
            exit(0);
        }

        let parsed = parser.parse(&matches);
        let mut config = parsed.0;
        let success_msg = parsed.1;

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
}
