use std::env::args_os;
use std::process::exit;

use crate::command_parser::CommandMatchParser;
use crate::output_handler::{handle_error, handle_output};
// use clap::AppSettings::{ArgRequiredElseHelp, SubcommandRequiredElseHelp};
use clap::{load_yaml, App};
use sap_adt_bindings::config::program_config::{
    ConfigCopyDatabaseTable, ConfigGetTableDetails, Program,
};
use sap_adt_bindings::config::{CopyToSys, Create, Delete, SendWith, Sendable};
use sap_adt_bindings::net::{Destination, SAPClient};

pub mod command_parser;
pub mod output_handler;

use sap_adt_bindings::app_config::AppConfig;

// trait xxx {
//     type abc;

//     fn x() -> <Self as xxx>::abc;
// }

// struct xyz {}
// impl xxx for xyz {
//     type abc = String;
//     fn x() -> String {
//         String::new()
//     }
// }

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
    // let mut config = ConfigCopyDatabaseTable::new(
    //     "ZHRE_0024",
    //     &Destination {
    //         host: String::from("https://hamerpiea.zalaris.de"),
    //         port: 443,
    //         sys_id: String::from("IEA"),
    //         uname: String::from("PFRANK"),
    //         passwd: String::from("Start1234$"),
    //         mandt: String::from("200"),
    //         lang: String::from("DE"),
    //     },
    //     "Z_PFRANK",
    //     "IEAK908900",
    // );

    let mut app_conf = AppConfig::init();
    // let mut prog = Program::new("ZPF_1511_2", None, None);
    // let mut config = prog.create();
    // let mut config = CommandMatchParser::new(&app_conf).parse(&matches);
    let mut prog = Program::new("ZPF_1451", None, None);
    let dest2 = Destination {
        host: String::from("https://hamerpiea.zalaris.de"),
        port: 443,
        sys_id: String::from("IEA"),
        uname: String::from("PFRANK"),
        passwd: String::from("Start1234$"),
        mandt: String::from("200"),
        lang: String::from("DE"),
    };

    let mut client: SAPClient;

    let dest = app_conf.get_default_destination();
    // println!("{:?}", dest);
    let update_session_file: bool;

    if let Some(session) = app_conf.get_session_from_sys(&dest.sys_id) {
        client = SAPClient::from_session(&dest, session);
        update_session_file = false;
    } else {
        client = SAPClient::new(&dest);
        update_session_file = true;
    }

    prog.delete().send_with(&mut client).await;
    // prog.copy_to_sys(&dest2).send_with(&mut client).await;
    // let mut x = prog.delete();
    // let a = x.send_with(&mut client).await;
    // prog.create().send_with(&mut client).await;
    // let mut xxx = prog.copy_to_sys(&dest2);
    // let mut xxx = prog.create();
    // xxx.send_with(&mut client).await;
    // prog.copy_to_sys(&dest).send_with(&mut client);
    // config.send_with(&mut client);
    // match &config.send_with(&mut client).await {
    //     Ok(()) => std::process::exit(0), //handle_output(config.get_response().unwrap()),
    //     Err(e) => handle_error(e),
    // }

    if update_session_file {
        app_conf.set_session_for_sys("ITK", &client.get_session().unwrap());
    }
    app_conf.update_file();
}
