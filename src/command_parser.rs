use std::{any::Any, marker::PhantomData};

use async_trait::async_trait;
use clap::ArgMatches;
use sap_bindings::{
    config::AppConfig,
    net::{
        behavior::{CopyTo, CopyToSys, Create, Delete},
        object::{Class, Program},
        SendWith,
    },
};
// use sap_adt_bindings::{
//     app_config::AppConfig,
//     config::{
//         class::Class,
//         class_config::{ClassConfig, ClassError, ClassResponse},
//         freestyle_config::{FreeStyleConfig, FreeStyleError, FreeStyleResponse},
//         program::Program,
//         AdtError, AdtResponse, Config, CopyTo, CopyToSys, Create, DefaultResponse, Delete, Request,
//         Response, Responses, SAPClient, SendWith, Sendable, SendableConfig,
//     },
//     data::abap_table::ABAPTable,
//     net::Destination,
// };

pub mod command_match_parser {}

pub struct CommandMatchParser<'a> {
    config: &'a AppConfig,
    prog: Option<Program>, // res: Option<T>,
                           // result: Option<U>, // program: Program
}

pub struct TableCommandParser {}
pub struct SqlCommandParser {}
pub struct ProgramCommandParser {}
pub struct ClassCommandParser {}

impl<'a> CommandMatchParser<'a> {
    pub fn new(config: &'a AppConfig) -> Self {
        CommandMatchParser {
            config,
            prog: None
            // res: None,
            // result: None,
        }
    }

    pub fn parse<'b>(&'b mut self, args: &ArgMatches) -> Box<dyn SendWith + 'b>
// where T: Response
    {
        // type x = <C as Sendable<FreeStyleResponse, FreeStyleError>>

        match &args.subcommand() {
            // &Some(("table", matches)) => parse_table_command(matches),
            // &Some(("sql", matches)) => parse_sql_command(matches),
            &Some(("new", new_matches)) => match new_matches.subcommand() {
                Some(("prog", matches)) => Program::new(
                    matches.value_of("name").unwrap(),
                    matches.value_of("package"),
                    matches.value_of("transport"),
                )
                .create(),

                // Some(("class", matches)) => parse_class_command(matches),
                // Some(("tab", matches)) => parse_new_table_command(matches),
                Some(("class", matches)) => Class::new(
                    matches.value_of("name").unwrap(),
                    matches.value_of("transport"),
                    matches.value_of("package"),
                )
                .create(),
                Some((_, _)) => panic!(""),
                None => panic!(""),
            },
            &Some(("copy", matches)) => {
                match matches.subcommand() {
                    Some(("prog", matches)) => {
                        self.prog = Some(Program::new(
                            matches.value_of("source").unwrap(),
                            matches.value_of("package"),
                            matches.value_of("transport"),
                        ));

                        return if matches.is_present("destination") {
                            self.prog.as_ref().unwrap().copy_to_sys(
                                &self
                                    .config
                                    .get_destination_from_sys(
                                        &matches.value_of("destination").unwrap(),
                                    )
                                    .unwrap(),
                            )
                        } else {
                            self.prog
                                .as_ref()
                                .unwrap()
                                .copy_to(matches.value_of("source").unwrap())
                        };

                        // *a
                    }
                    // Some(("tab", matches)) => self.parse_copy_database_command(matches),
                    Some((&_, _)) => panic!(""),
                    None => panic!(""),
                }
            }
            &Some(("delete", delete_matches)) => match delete_matches.subcommand() {
                Some(("prog", matches)) => Program::new(
                    matches.value_of("name").unwrap(),
                    matches.value_of("package"),
                    matches.value_of("transport"),
                )
                .delete(),

                // Some(("class", matches)) => parse_class_command(matches),
                // Some(("tab", matches)) => parse_new_table_command(matches),
                Some((_, _)) => panic!(""),
                None => panic!(""),
            },
            // &Some(("delete", matches)) => parse_delete_program_command(matches),
            &Some((_, _)) => panic!(""),
            None => panic!(""),
        }
    }
}
