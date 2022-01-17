use std::{any::Any, marker::PhantomData};

use async_trait::async_trait;
use clap::ArgMatches;

use i18n_codegen::i18n;
use sap_bindings::{
    config::AppConfig,
    net::{
        behavior::{CopyTo, CopyToSys, Create, Delete},
        object::{Class, Program, Table},
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
i18n!("./src/i18n");
pub mod command_match_parser {}

pub struct CommandMatchParser<'a> {
    config: &'a AppConfig,
    prog: Option<Program>,
    tab: Option<Table>,
    success_msg: String,
    locale: Locale, // res: Option<T>,
                    // result: Option<U>, // program: Program
}

// pub struct TableCommandParser {}
// pub struct SqlCommandParser {}
// pub struct ProgramCommandParser {}
// pub struct ClassCommandParser {}

impl<'a> CommandMatchParser<'a> {
    pub fn get_success_smg(&self) -> String {
        self.success_msg.clone()
    }

    pub fn new(config: &'a AppConfig, lang: &str) -> Self {
        CommandMatchParser {
            config,
            prog: None,
            success_msg: String::from(""),
            locale: match lang {
                "DE" => Locale::De,
                _ => Locale::En
            },
            tab: None
            // res: None,
            // result: None,
        }
    }
    pub fn is_check_command(&self, matches: &ArgMatches) -> bool {
        match matches.subcommand() {
            Some(("check", matches)) => true,
            _ => false,
        }
    }
    pub fn is_settings_command(&self, matches: &ArgMatches) -> bool {
        match matches.subcommand() {
            Some(("settings", matches)) => true,
            _ => false,
        }
    }
    pub fn is_dest_command(&self, matches: &ArgMatches) -> bool {
        match matches.subcommand() {
            Some(("dest", matches)) => true,
            _ => false,
        }
    }
    pub fn parse<'b>(&'b mut self, args: &ArgMatches) -> (Box<dyn SendWith + 'b>, Option<String>)
// where T: Response
    {
        // i18n!("./i18n");
        // type x = <C as Sendable<FreeStyleResponse, FreeStyleError>>
        // Locale::De::created();
        match &args.subcommand() {
            // &Some(("table", matches)) => parse_table_command(matches),
            // &Some(("sql", matches)) => parse_sql_command(matches),
            &Some(("tab", matches)) => (
                Table::freestyle(
                    &format!("SELECT * FROM {}", matches.value_of("name").unwrap()),
                    matches.value_of_t("rows").ok(),
                ),
                None,
            ),

            &Some(("new", new_matches)) => match new_matches.subcommand() {
                Some(("prog", matches)) => {
                    // self.success_msg = self.locale.created(Name(matches.value_of("name").unwrap()));
                    (
                        Program::new(
                            matches.value_of("name").unwrap(),
                            matches.value_of("package"),
                            matches.value_of("transport"),
                        )
                        .create(),
                        Some(self.locale.created(Name(matches.value_of("name").unwrap()))),
                    )
                }

                // Some(("class", matches)) => parse_class_command(matches),
                // Some(("tab", matches)) => parse_new_table_command(matches),
                Some(("class", matches)) => (
                    Class::new(
                        matches.value_of("name").unwrap(),
                        matches.value_of("transport"),
                        matches.value_of("package"),
                    )
                    .create(),
                    Some(self.locale.created(Name(matches.value_of("name").unwrap()))),
                ),
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
                            (
                                self.prog.as_ref().unwrap().copy_to_sys(
                                    &self
                                        .config
                                        .get_destination_from_sys(
                                            &matches.value_of("destination").unwrap(),
                                        )
                                        .unwrap(),
                                ),
                                Some(self.locale.copied(From(""), To(""))),
                            )
                        } else {
                            (
                                self.prog
                                    .as_ref()
                                    .unwrap()
                                    .copy_to(matches.value_of("source").unwrap()),
                                Some(String::from("table successfully copied")),
                            )
                        };

                        // *a
                    }

                    Some(("tab", matches)) => {
                        self.tab = Some(Table::new(
                            matches.value_of("source").unwrap(),
                            matches.value_of("transport"),
                            matches.value_of("package"),
                        ));

                        return if matches.is_present("destination") {
                            (
                                self.tab.as_ref().unwrap().copy_to_sys(
                                    &self
                                        .config
                                        .get_destination_from_sys(
                                            &matches.value_of("destination").unwrap(),
                                        )
                                        .unwrap(),
                                ),
                                Some(self.locale.copied(From(""), To(""))),
                            )
                        } else {
                            (
                                self.prog
                                    .as_ref()
                                    .unwrap()
                                    .copy_to(matches.value_of("source").unwrap()),
                                Some(self.locale.copied(From(""), To(""))),
                            )
                        };
                    }
                    // Some(("tab", matches)) => self.parse_copy_database_command(matches),
                    Some((&_, _)) => panic!(""),
                    None => panic!(""),
                }
            }
            &Some(("delete", delete_matches)) => match delete_matches.subcommand() {
                Some(("prog", matches)) => {
                    self.success_msg = self.locale.deleted(Name(matches.value_of("name").unwrap()));
                    (
                        Program::new(
                            matches.value_of("name").unwrap(),
                            matches.value_of("package"),
                            matches.value_of("transport"),
                        )
                        .delete(),
                        Some(self.locale.deleted(Name(matches.value_of("name").unwrap()))),
                    )
                }

                // Some(("class", matches)) => parse_class_command(matches),
                // Some(("tab", matches)) => parse_new_table_command(matches),
                Some((_, _)) => panic!(""),
                None => panic!(""),
            },
            // &Some(("dest", matches)) => AppConfig::open_destination_file(),

            // &Some(("delete", matches)) => parse_delete_program_command(matches),
            &Some((_, _)) => std::process::exit(0),
            None => panic!(""),
        }
    }
}
