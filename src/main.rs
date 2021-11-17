use std::io::Read;
use std::iter::repeat;
use std::process::exit;

use crate::command_parser::CommandMatchParser;
use crate::output_handler::{handle_error, handle_output};
use ::aes::cipher::AsyncStreamCipher;
// use aes::cipher::generic_array::typenum::{Len, UInt, UTerm, B0, B1};
use aes::cipher::generic_array::ArrayLength;
// use aes::cipher::generic_array::GenericArray;
// use aes::{BlockEncrypt, NewBlockCipher};
use app_config::AppConfig;
use clap::{load_yaml, App};

use crypto::aes::KeySize;
use crypto::buffer::{ReadBuffer, RefReadBuffer, RefWriteBuffer, WriteBuffer};
use sap_adt_bindings::config::program_config::{
    ConfigCopyProgramToSys, ConfigGetProgramSource, ConfigUpdateProgramSource,
};
use sap_adt_bindings::config::{Config, Sendable};
use sap_adt_bindings::net::SAPClient;
pub mod app_config;
pub mod command_parser;
pub mod crypt;
pub mod output_handler;
// use crypto::aes::ecb_encryptor;
// use crypto::aes::{self, KeySize};
// pub use aead;
pub use aes;
use crypto::blowfish::Blowfish;
use crypto::symmetriccipher::{Decryptor, Encryptor, SynchronousStreamCipher};
use rand::rngs::OsRng;
use rand::Rng;
use rand::{CryptoRng, RngCore};
use rustc_serialize::base64::{ToBase64, STANDARD};

use aes::cipher::{
    generic_array::GenericArray, BlockCipher, BlockDecrypt, BlockEncrypt, NewBlockCipher,
};
use aes::{Aes128, Block, ParBlocks};
use aes_gcm::aead::heapless::Vec;
use aes_gcm::aead::{AeadInPlace, NewAead};
use aes_gcm::{Aes256Gcm, Key, Nonce};
use crypt::Crypt; // Or `Aes128Gcm`
                  // pub fn decrypt_aes128(key: &[u8], iv: &[u8], data: &[u8]) -> Vec<u8> {
                  //     let mut encrypted_data = data.clone().to_owned();
                  //     let cipher = Aes128::new_from_slice(&key).unwrap();
                  //     cipher.decrypt(&mut encrypted_data).unwrap().to_vec()
                  // }
fn hash() {
    let mut crypt = Crypt::new_random();

    let encrypt_base64 = crypt.encrypt("My name a jeff!!!");

    let original_passwd = crypt.decrypt(&encrypt_base64);

    println!("Passwort: {}", original_passwd);
    // let mut key_raw: Vec<u8, 32> = repeat(0u8).take(32).collect();
    // OsRng.fill_bytes(&mut key_raw[..]);
    // let key = Key::from_slice(&key_raw);

    // let mut nonce_raw: Vec<u8, 12> = repeat(0u8).take(12).collect();
    // OsRng.fill_bytes(&mut nonce_raw[..]);
    // // let key = Key::from_slice(b"an example very very secret key.");
    // let cipher = Aes256Gcm::new(key);
    // let nonce = Nonce::from_slice(&nonce_raw);
    // // let nonce = Nonce::from_slice(b"unique nonce"); // 96-bits; unique per message

    // let mut buffer: Vec<u8, 128> = Vec::new(); // Buffer needs 16-bytes overhead for GCM tag
    // buffer.extend_from_slice(b"plaintext messagexxx");

    // // Encrypt `buffer` in-place, replacing the plaintext contents with ciphertext
    // cipher
    //     .encrypt_in_place(nonce, key, &mut buffer)
    //     .expect("encryption failure!");

    // // `buffer` now contains the message ciphertext
    // println!("{:?}", buffer.to_base64(STANDARD));

    // // Decrypt `buffer` in-place, replacing its ciphertext context with the original plaintext
    // cipher
    //     .decrypt_in_place(nonce, key, &mut buffer)
    //     .expect("decryption failure!");
    // // assert_eq!(&buffer, b"plaintext message");
    // println!("{:?}", String::from_utf8(buffer.to_vec()));
    // let secret = "My name a jeffff";
    // let x = &secret[..];
    // let len = secret.chars().count();
    // // let xxx = GenericArray::default()
    // // let len = ;
    // let x = vec![0u8; 14];

    // // let key = GenericArray::from_slice(&[x]);
    // let key: &GenericArray<u8, UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>> =
    //     GenericArray::from_slice(&[0u8; 16]);
    // // key.fill(value)
    // let mut block = Block::from_slice(secret.as_bytes()).clone();
    // let mut block8 = ParBlocks::default();
    // // struct Len {}
    // // unsafe impl<T> ArrayLength<T> for Len {
    // //     #[doc(hidden)]
    // //     type ArrayType = [T; 14];
    // // }
    // // Initialize cipher
    // let cipher = Aes128::new_from_slice(&key).unwrap();

    // let block_copy = block.clone();

    // // Encrypt block in-place
    // cipher.encrypt_block(&mut block);

    // println!("{:?}", &block);
    // // And decrypt it back
    // cipher.decrypt_block(&mut block);

    // println!("{:?}", String::from_utf8(block.to_vec()));
    // // assert_eq!(block, block_copy);
    // println!("{:?}", &block);
    // // We can encrypt 8 blocks simultaneously using
    // // instruction-level parallelism
    // let block8_copy = block8.clone();
    // cipher.encrypt_par_blocks(&mut block8);
    // cipher.decrypt_par_blocks(&mut block8);
    // assert_eq!(block8, block8_copy);
    // let mut gen = CryptoRng::new().expect("Failed to get OS random generator");
    // let mut key: Vec<u8> = repeat(0u8).take(16).collect();
    // OsRng.fill_bytes(&mut key[..]);
    // let mut nonce: Vec<u8> = repeat(0u8).take(16).collect();
    // OsRng.fill_bytes(&mut nonce[..]);
    // println!("Key: {:?}", key);
    // println!("Nonce: {:?}", nonce);
    // let mut cipher = aes::ctr(KeySize::KeySize128, &key, &nonce);
    // let secret = "I like Soda";
    // println!(
    //     "Must match this==> {}",
    //     secret.as_bytes().to_base64(STANDARD)
    // );
    // let mut output: Vec<u8> = repeat(0u8).take(secret.len()).collect();

    // let mut read_buffer = RefReadBuffer::new(secret.as_bytes());
    // let mut write_buffer = RefWriteBuffer::new(&mut output);

    // cipher.encrypt(&mut read_buffer, &mut write_buffer, true);

    // // cipher.process(secret.as_bytes(), &mut output[..]);
    // println!("Ciphertext: {:?}", output.to_base64(STANDARD));
    // let mut new_output: Vec<u8> = repeat(0u8).take(secret.len()).collect();
    // let mut x = new_output.clone();
    // let mut new_output_for_buffer = 0u8;
    // let mut read_buffer = RefReadBuffer::new(&output);
    // let mut write_buffer = RefWriteBuffer::new(&mut new_output);

    // cipher
    //     .decrypt(&mut read_buffer, &mut write_buffer, true)
    //     .unwrap();

    // // let decryptor = cipher.decryptor();
    // // new_output = repeat(0u8).take(secret.len()).collect();
    // let mut result = write_buffer.take_read_buffer();
    // let mut bytes = result.take_remaining().read_to_end(&mut x).unwrap();
    // let mut result_output: String = String::new();
    // // bytes.to_base64(STANDARD);
    // println!("Password was: {:?}", &new_output.to_base64(STANDARD));
}

#[tokio::main]
async fn main() {
    // let key = GenericArray::from_slice(&[0u8; 16]);
    // println!("Key: {:?}", &key);
    // let cipher = aes::Aes256::new(&key);
    // hash();
    // exit(0);
    // let mut block = aes::Block::from_slice("Start123!".as_bytes()).clone();
    // println!("Block: {:?}", block);
    // cipher.encrypt_block(&mut block);
    // println!("Block: {:?}", block);
    let cli_yaml = load_yaml!("cli.yaml");

    let matches = App::from(cli_yaml).get_matches();
    // std::env::set_var("$JUNK", "xxx");
    // let mut config = ConfigCopyProgramToSys::new("ZPF_1511");
    let mut config = CommandMatchParser::parse(&matches);
    // let mut config = ConfigGetProgramSource::new("ZPF_FLIGHT");
    let mut app_conf = AppConfig::init();
    let mut client: SAPClient;

    // let host = env::var("DEFAULT").unwrap();
    // let host = "http://hamerpitk01.lej.it2-solutions.com:8000";
    let dest = app_conf.get_default_destination();
    let update_session_file: bool;

    if let Some(session) = app_conf.get_session_from_sys(&dest.sys_id) {
        client = SAPClient::from_session(&dest, session);
        update_session_file = false;
    } else {
        client = SAPClient::new(&dest);
        update_session_file = true;
    }
    // println!("{:?}", config);
    // println!("{}", config.get_path());
    // println!("{}", config.get_body());

    match config.send_with(&mut client).await {
        Ok(()) => handle_output(config.get_response().unwrap()),
        Err(e) => handle_error(e),
    }
    // println!("{}", config.get_source().unwrap());
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
