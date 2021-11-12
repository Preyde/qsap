use clap::ArgMatches;
use sap_adt_bindings::config::{
    class_config::{ClassConfig, ClassError, ClassResponse},
    freestyle_config::{FreeStyleConfig, FreeStyleError, FreeStyleResponse},
    program_config::{ConfigCreateProgram, ProgramError, ProgramResponse},
    AdtError, AdtResponse, Config, Responses, Sendable,
};

pub mod command_match_parser {}

pub struct CommandMatchParser {}

// struct TableCommand {
//     config: FreeStyleConfig,
// }
// impl TableCommand {
//     fn new(matches: ArgMatches) -> Self {
//         TableCommand {
//             config: FreeStyleConfig::new(
//                 format!("SELECT * FROM {}", matches.value_of("name").unwrap()),
//                 matches.value_of_t("rows").ok(),
//             ),
//         }
//     }
//     fn get_config(&self) -> FreeStyleConfig {
//         self.config
//     }
// }
pub trait CommandParser<C, T, E>
where
    C: Config + Sendable<T, E>,
    T: AdtResponse,
    E: AdtError,
{
    fn parse(args: &ArgMatches) -> C;
}
pub struct TableCommandParser {}
pub struct SqlCommandParser {}
pub struct ProgramCommandParser {}
pub struct ClassCommandParser {}
// type SendableConfig = dyn Config + SendWith;
// trait SendableConfig: Config + SendWith {}

impl CommandParser<FreeStyleConfig, FreeStyleResponse, FreeStyleError> for TableCommandParser {
    fn parse(args: &ArgMatches) -> FreeStyleConfig {
        let tab_name = args.value_of("name").unwrap();
        let rows: Option<u32> = args.value_of_t("rows").ok();
        // let path = args.value_of("out");
        FreeStyleConfig::new(format!("SELECT * FROM {0}", tab_name), rows)
    }
}
impl CommandParser<FreeStyleConfig, FreeStyleResponse, FreeStyleError> for SqlCommandParser {
    fn parse(args: &ArgMatches) -> FreeStyleConfig {
        FreeStyleConfig::new(
            args.value_of_t("sql_exp").unwrap(),
            args.value_of_t("rows").ok(),
        )
    }
}
impl CommandParser<ConfigCreateProgram, ProgramResponse, ProgramError> for ProgramCommandParser {
    fn parse(args: &ArgMatches) -> ConfigCreateProgram {
        ConfigCreateProgram::new(
            args.value_of("name").unwrap(),
            args.value_of("package").unwrap(),
            args.value_of("transport").unwrap(),
        )
    }
}
impl CommandParser<ClassConfig, ClassResponse, ClassError> for ClassCommandParser {
    fn parse(args: &ArgMatches) -> ClassConfig {
        ClassConfig::new(
            args.value_of("name").unwrap(),
            args.value_of("package").unwrap(),
            args.value_of("transport").unwrap(),
        )
    }
}

impl CommandMatchParser {
    pub fn new() -> CommandMatchParser {
        CommandMatchParser {}
    }
}
// trait NewTrait: Config + SendWith {}

enum CommandMatchOutput {
    FreeStyle(FreeStyleConfig),
    ProgramCreate(ConfigCreateProgram),
    ClassCreate(ClassConfig),
}

enum Commands {
    TableCommand(FreeStyleConfig),
    ProgramCommand(ConfigCreateProgram),
    ClassCommand(ClassConfig),
}

// impl CommandParser for CommandMatchParser {
//     fn parse(args: &ArgMatches) -> C {}
// }

impl CommandMatchParser {
    pub fn parse<C, T, E>(args: &ArgMatches) -> C
    where
        C: Config + Sendable<T, E>,
        T: AdtResponse,
        E: AdtError, //     C: Config<_, _>,
                     //     T: AdtResponse<Responses>,
                     //     E: AdtError
    {
        match &args.subcommand() {
            &Some(("table", matches)) => Box::new(TableCommandParser::parse(matches)),
            &Some(("sql", matches)) => Box::new(SqlCommandParser::parse(matches)),
            &Some(("new", new_matches)) => match new_matches.subcommand() {
                Some(("prog", matches)) => Box::new(ProgramCommandParser::parse(matches)),
                Some(("class", matches)) => Box::new(ClassCommandParser::parse(matches)),

                Some((_, _)) => panic!(""),
                None => panic!(""),
            },
            &Some((_, _)) => panic!(""),
            None => panic!(""),
        }
    }
}

// impl CommandParser for CommandMatchParser {
//     fn parse(args: &ArgMatches) -> T {
//         match &args.subcommand() {
//             &Some(("table", matches)) => TableCommandParser::parse(matches),
//             &Some(("sql", matches)) => SqlCommandParser::parse(matches),
//             &Some(("new", new_matches)) => match new_matches.subcommand() {
//                 Some(("prog", matches)) => ProgramCommandParser::parse(matches),
//                 Some(("class", matches)) => ClassCommandParser::parse(matches),

//                 Some((_, _)) => panic!(""),
//                 None => panic!(""),
//             },
//             // &Some(("copy", copy_matches)) => {
//             //     let source_name: String = copy_matches.value_of_t("source").unwrap();
//             //     let prog_name: String = copy_matches.value_of_t("name").unwrap();
//             //     let package_name: String = copy_matches.value_of_t("package").unwrap();
//             //     let transport_request: String = copy_matches.value_of_t("transport").unwrap();

//             //     copy_program(&source_name, &prog_name, &transport_request, &package_name).await;
//             // }
//             // &Some(("transport", transport_matches)) => {}
//             // &Some(("delete", delete_matches)) => {
//             //     let prog_name: String = delete_matches.value_of_t("name").unwrap();

//             //     delete_program(&prog_name).await;
//             // }
//             &Some((_, _)) => panic!(""),
//             None => panic!(""),
//         }
//     }
// }
