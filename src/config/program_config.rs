use std::fmt::Error;

use crate::{
    data::{
        abap_prog::AbapProg,
        abap_table::{Dd02v, Dd03pStruc, Dd09l, SoapResponse, XmlTest, DD03P_TAB},
    },
    net::Destination,
};

use super::{
    strategy::{CopyToSysStrategy, DefaultStrategy, LockStrategy},
    AdtError, AdtResponse, Config, CopyTo, CopyToSys, Create, DefaultResponse, Delete, Lock,
    LockHandle, LockObject, Request, Response, Responses, SAPClient, SendWith, Sendable,
    SendableConfig, Source,
};
// use crate::config::Sendable;
use async_trait::async_trait;
use format_xml::xml;
use reqwest::Method;
use serde::Deserialize;
// pub trait Config {
//     fn get_path(&self) -> &String;
//     fn get_body(&self) -> &String;
// }
#[derive(Debug, Deserialize)]
#[serde(rename = "DATA")]
pub struct LockHandleData {
    pub LOCK_HANDLE: String,
    pub CORRNR: String,
    pub CORRUSER: String,
    pub CORRTEXT: String,
}

#[derive(Debug, Deserialize)]
pub struct LockHandleValues {
    pub DATA: LockHandleData,
}
#[derive(Debug, Deserialize)]
#[serde(rename = "asx:abap")]
pub struct LockHandleResponse {
    // #[serde(rename = "asx:values")]
    pub values: LockHandleValues,
}
// impl AdtResponse<Responses> for ProgramResponse {
//     fn get_data(&self) -> Responses {
//         Responses::Program(ProgramResponse::)
//     }
// }
// impl AdtResponse for ProgramResponse {
//     fn get_data(self) -> Responses {
//         Responses::Program(String::from(""))
//     }
// }

// #[derive(Debug)]
// pub struct ConfigCreateProgram {
//     body: String,
//     path: String,
//     prog_name: String,
//     text: Option<String>,
// }
// #[derive(Debug)]
// pub struct ConfigDeleteProgram {
//     body: String,
//     path: String,
//     prog_name: String,
//     lock_handle: Option<String>,
//     transport_request: String,
// }
// #[derive(Debug)]
// pub struct ConfigCopyProgram {
//     body: String,
//     path: String,
//     prog_name: String,
// }
// #[derive(Debug)]
// pub struct ConfigUpdateProgramSource {
//     body: String,
//     prog_name: String,
//     lock_handle: Option<String>,
// }
// #[derive(Debug)]
// pub struct ConfigCopyProgramToSys {
//     prog_name: String,
// }

// impl ConfigCopyProgramToSys {
//     pub fn new(prog_name: &str) -> Self {
//         ConfigCopyProgramToSys {
//             prog_name: String::from(prog_name),
//         }
//     }
// }
#[derive(Debug)]
pub struct Program {
    body: String,
    path: String,
    prog_name: String,
    package_name: Option<String>,
    transport_request: Option<String>,
    lock_handle: Option<String>,
    source: Option<String>,
}
// impl SendableConfig for Program {}
// #[async_trait]
// impl Sendable for Program {
//     async fn send_with(&mut self, client: &mut SAPClient) -> Result<(), AdtError> {
//         let res = client.send(self).await;
//         // println!("{}", res.status());
//         // println!("{}", res.text().await.unwrap());
//         Ok(())
//     }
//     fn get_response(&self) -> Option<Responses> {
//         Some(Responses::Program(String::from("Program copied")))
//     }
// }
// impl <T: Config> T for Program<T> {

// }

impl Config for Program {
    fn get_body(&self) -> String {
        self.body.clone()
    }
    fn get_path(&self) -> String {
        self.path.clone()
    }
}
// trait Xxx: Config {
//     fn do_xxx(&self);
// }

// struct abc {}

// impl Xxx for abc {
//     fn do_xxx<T>(x: T)
//     where
//         T: Xxx,
//     {
//         x.xxx();
//     }
//     fn do_xxx<T>(x: T)
//     where
//         T: Config,
//     {
//         x.xxx();
//     }
// }

// fn do_xxx<T>(x: T)
// where
//     T: Xxx,
// {
//     x.xxx();
// }
// #[async_trait]
// impl<T> SendWith for T
// where
//     T: Request,
// {
//     async fn send_with(&mut self, client: &mut SAPClient) -> Result<(), AdtError> {
//         let res = client.request(self).await;

//         // println!("{}", res.status());

//         // self.source = res.text().await.ok();
//         Ok(())
//     }
// }

// struct DefaultStrategy {
//     body: String,
//     path: String,
//     method: Method,
// }
// #[async_trait]
// impl Request for DefaultStrategy {
//     fn get_body(&self) -> String {
//         self.body.clone()
//     }
//     fn get_path(&self) -> String {
//         self.path.clone()
//     }
//     fn get_method(&self) -> reqwest::Method {
//         Method::GET
//     }
//     async fn send_with(&mut self, client: &mut SAPClient) -> Result<(), AdtError> {
//         let res = client.request(self).await;

//         // println!("{}", res.status());

//         // self.source = res.text().await.ok();
//         Ok(())
//     }
// }

// struct LockStrategy<'a> {
//     body: String,
//     path: String,
//     prog: &'a mut Program,
// }
// #[async_trait]
// impl<'a> Request for LockStrategy<'a> {
//     fn get_body(&self) -> String {
//         self.body.clone()
//     }
//     fn get_path(&self) -> String {
//         self.path.clone()
//     }
//     fn get_method(&self) -> reqwest::Method {
//         Method::DELETE
//     }
//     async fn send_with(&mut self, client: &mut SAPClient) -> Result<(), AdtError> {
//         client.set_stateful(true);

//         let lock_handle_res = client.lock(self.prog).await;

//         let xml = lock_handle_res.text().await.unwrap();
//         // println!("{:?}", &xml);
//         let lock_handle: LockHandleResponse = quick_xml::de::from_str(&xml).unwrap();

//         self.prog
//             .set_lock_handle(&lock_handle.values.DATA.LOCK_HANDLE);

//         let res = client.request(self).await;
//         // println!("{}", res.text().await.unwrap());
//         Ok(())
//     }
// }

// struct CopyToSysStrategy {
//     body: String,
//     path: String,
// }
// #[async_trait]
// impl Request for CopyToSysStrategy {
//     fn get_body(&self) -> String {
//         self.body.clone()
//     }
//     fn get_path(&self) -> String {
//         self.path.clone()
//     }
//     fn get_method(&self) -> reqwest::Method {
//         Method::GET
//     }
//     async fn send_with(&mut self, client: &mut SAPClient) -> Result<(), AdtError> {
//         let mut conf_get_source = ConfigGetProgramSource::new(&self.prog_name);

//         conf_get_source.send_with(client).await?;

//         client.set_destination(&Destination {
//             host: String::from("https://hamerpiea.zalaris.de"),
//             port: 443,
//             sys_id: String::from("IEA"),
//             uname: String::from("PFRANK"),
//             passwd: String::from("Start1234$"),
//             mandt: String::from("200"),
//             lang: String::from("DE"),
//         });

//         // println!("{}", &conf_get_source.get_source().unwrap());
//         client.set_stateful(true);
//         client.clear_session();
//         ConfigCreateProgram::new(&self.prog_name, None, None)
//             .send_with(client)
//             .await?;

//         ConfigUpdateProgramSource::new(&self.prog_name, &conf_get_source.get_source().unwrap())
//             .send_with(client)
//             .await?;

//         Ok(())
//     }
// }

impl AsRef<Program> for Program {
    fn as_ref(&self) -> &Program {
        self
    }
}

impl AsMut<Program> for Program {
    fn as_mut(&mut self) -> &mut Program {
        self
    }
}

impl Program {
    pub fn new(
        prog_name: &str,
        package_name: Option<&str>,
        transport_request: Option<&str>,
    ) -> Self {
        Program {
            body: String::new(),
            path: String::new(),
            prog_name: prog_name.to_string(),
            package_name: match package_name {
                Some(v) => Some(v.to_string()),
                None => None,
            },
            transport_request: match transport_request {
                Some(v) => Some(v.to_string()),
                None => None,
            },
            lock_handle: None,
            source: None,
        }
    }
}
impl CopyToSys<DefaultResponse, String> for Program {
    fn copy_to_sys<'a>(&'a self, dest: &Destination) -> Box<dyn SendWith<DefaultResponse> + 'a> {
        Box::new(CopyToSysStrategy::new(
            // LockObject::new(&"/sap/bc/adt/programs/programs/"),
            self,
            dest.clone(),
        ))
    }
}

impl Source<DefaultResponse, String> for Program {
    fn source(&self) -> Box<dyn SendWith<DefaultResponse>> {
        Box::new(DefaultStrategy::new(
            String::new(),
            format!(
                "/sap/bc/adt/programs/programs/{}/source/main",
                self.prog_name
            ),
            Method::GET,
        ))
    }
    fn update_source(&self, source: &str) -> Box<dyn SendWith<DefaultResponse>> {
        Box::new(LockStrategy::new(
            source.to_string(),
            // source.to_string(),
            // format!(
            //     "/sap/bc/adt/programs/programs/{}/source/main?lockHandle={}",
            //     self.prog_name,
            //     self.lock_handle.as_ref().unwrap()
            // ),
            Method::PUT,
            LockObject::new(&format!(
                "/sap/bc/adt/programs/programs/{}/source/main",
                self.prog_name,
            )),
        ))
    }
    fn get_source(&self) -> String {
        String::new()
    }
}

impl Create<DefaultResponse, String> for Program {
    fn create(&self) -> Box<dyn SendWith<DefaultResponse>> {
        Box::new(DefaultStrategy::new(
            format!(
                r#"<?xml version="1.0" encoding="UTF-8"?>
                                    <program:abapProgram xmlns:program="http://www.sap.com/adt/programs/programs" xmlns:adtcore="http://www.sap.com/adt/core" 
                                        adtcore:description="Von Programm erstellt" adtcore:language="DE" adtcore:name="{program_name}" adtcore:type="PROG/P" adtcore:masterLanguage="DE" 
                                        adtcore:masterSystem="IEA" adtcore:responsible="PFRANK">  
                                    
                                        <adtcore:packageRef adtcore:name="{package_name}"/>
                                </program:abapProgram>"#,
                program_name = self.prog_name,
                package_name = self.package_name.as_ref().unwrap_or(&String::from("$TMP"))
            ),
            match &self.transport_request {
                Some(t) => format!("/sap/bc/adt/programs/programs?corrNr={0}", t),
                None => String::from("/sap/bc/adt/programs/programs"),
            },
            Method::POST,
        ))
        //ITKK901409
        // self.path = match self.transport_request {
        //     Some(t) => format!("/sap/bc/adt/programs/programs?corrNr={0}", t),
        //     None => String::from("/sap/bc/adt/programs/programs"),
        // };

        // self.body = format!(
        //     r#"<?xml version="1.0" encoding="UTF-8"?>
        //                         <program:abapProgram xmlns:program="http://www.sap.com/adt/programs/programs" xmlns:adtcore="http://www.sap.com/adt/core"
        //                             adtcore:description="Von Programm erstellt" adtcore:language="DE" adtcore:name="{program_name}" adtcore:type="PROG/P" adtcore:masterLanguage="DE"
        //                             adtcore:masterSystem="ITK" adtcore:responsible="PFRANK">

        //                             <adtcore:packageRef adtcore:name="{package_name}"/>
        //                     </program:abapProgram>"#,
        //     program_name = self.prog_name,
        //     package_name = self.package_name.unwrap_or("$TMP")
        // );

        // self
    }
}

impl CopyTo<DefaultResponse, String> for Program {
    fn copy_to(&mut self, target_name: &str) -> Box<dyn SendWith<DefaultResponse>> {
        Box::new(DefaultStrategy::new(
            format!(
                r#"<?xml version="1.0" encoding="UTF-8"?><program:abapProgram xmlns:program="http://www.sap.com/adt/programs/programs" 
                    xmlns:abapsource="http://www.sap.com/adt/abapsource" 
                    xmlns:adtcore="http://www.sap.com/adt/core" adtcore:language="DE" adtcore:name="{prog_name}" 
                    adtcore:type="PROG/P" adtcore:masterLanguage="DE" adtcore:masterSystem="ITK" adtcore:responsible="PFRANK">
    
                        <adtcore:packageRef adtcore:name="{package_name}"/>
                        <abapsource:template abapsource:name="{source_prog}"/>
                    </program:abapProgram>"#,
                package_name = self.package_name.as_ref().unwrap_or(&String::from("$TMP")),
                source_prog = self.prog_name,
                prog_name = target_name
            ),
            match &self.transport_request {
                Some(t) => format!("/sap/bc/adt/programs/programs?corrNr={0}", t),
                None => String::from("/sap/bc/adt/programs/programs"),
            },
            Method::POST,
        ))

        // self.body = format!(
        //     r#"<?xml version="1.0" encoding="UTF-8"?><program:abapProgram xmlns:program="http://www.sap.com/adt/programs/programs"
        //         xmlns:abapsource="http://www.sap.com/adt/abapsource"
        //         xmlns:adtcore="http://www.sap.com/adt/core" adtcore:language="DE" adtcore:name="{prog_name}"
        //         adtcore:type="PROG/P" adtcore:masterLanguage="DE" adtcore:masterSystem="ITK" adtcore:responsible="PFRANK">

        //             <adtcore:packageRef adtcore:name="{package_name}"/>
        //             <abapsource:template abapsource:name="{source_prog}"/>
        //         </program:abapProgram>"#,
        //     package_name = self.package_name.unwrap_or("$TMP"),
        //     source_prog = self.prog_name,
        //     prog_name = target_name
        // );

        // self.path = if self.transport_request.is_some() {
        //     format!(
        //         "/sap/bc/adt/programs/programs?corrNr={}",
        //         self.transport_request.unwrap()
        //     )
        // } else {
        //     String::from("/sap/bc/adt/programs/programs")
        // };

        // self
    }
}

impl Lock for Program {
    fn lock() {}
}

impl Delete<DefaultResponse, String> for Program {
    fn delete(&mut self) -> Box<dyn SendWith<DefaultResponse>> {
        Box::new(LockStrategy::new(
            String::new(),
            Method::DELETE,
            LockObject::new(&format!("/sap/bc/adt/programs/programs/{}", self.prog_name)),
        ))
        // #[async_trait]
        // impl Sendable for Program {
        //     async fn send_with(&mut self, client: &mut SAPClient) -> Result<(), AdtError> {
        //         client.set_stateful(true);

        //         let lock_handle_res = client.lock(self).await;

        //         let xml = lock_handle_res.text().await.unwrap();
        //         // println!("{:?}", &xml);
        //         let lock_handle: LockHandleResponse = quick_xml::de::from_str(&xml).unwrap();

        //         self.lock_handle = Some(lock_handle.values.DATA.LOCK_HANDLE);

        //         let res = client.delete(self).await;
        //         // println!("{}", res.text().await.unwrap());
        //         Ok(())
        //     }
        //     fn get_response(&self) -> Option<Responses> {
        //         Some(Responses::Program(String::from("")))
        //     }
        // }
        // self.path = format!(
        //     "/sap/bc/adt/programs/programs/{0}?corrNr={1}&lockHandle={2}",
        //     self.prog_name,
        //     self.transport_request.unwrap(),
        //     self.lock_handle.as_ref().unwrap()
        // );
        // self
    }
}

// impl<'a> LockHandle for Program<'a> {
//     fn get_lock_path(&self) -> String {
//         format!(
//             "/sap/bc/adt/programs/programs/{}?_action=LOCK&accessMode=MODIFY",
//             self.prog_name
//         )
//     }
//     fn get_unlock_path(&self) -> Option<String> {
//         Some(format!(
//             "/sap/bc/adt/programs/programs/{}?_action=UNLOCK&lockHandle={}",
//             self.prog_name,
//             self.lock_handle.as_ref()?
//         ))
//     }
//     fn set_lock_handle(&mut self, lock_handle: &str) {
//         self.lock_handle = Some(lock_handle.to_string());
//     }
// }

// #[async_trait]
// impl Sendable for ConfigCopyProgramToSys {
//     async fn send_with(&mut self, client: &mut SAPClient) -> Result<(), AdtError> {
//         let mut conf_get_source = ConfigGetProgramSource::new(&self.prog_name);

//         conf_get_source.send_with(client).await?;

//         client.set_destination(&Destination {
//             host: String::from("https://hamerpiea.zalaris.de"),
//             port: 443,
//             sys_id: String::from("IEA"),
//             uname: String::from("PFRANK"),
//             passwd: String::from("Start1234$"),
//             mandt: String::from("200"),
//             lang: String::from("DE"),
//         });

//         // println!("{}", &conf_get_source.get_source().unwrap());
//         client.set_stateful(true);
//         client.clear_session();
//         ConfigCreateProgram::new(&self.prog_name, None, None)
//             .send_with(client)
//             .await?;

//         ConfigUpdateProgramSource::new(&self.prog_name, &conf_get_source.get_source().unwrap())
//             .send_with(client)
//             .await?;

//         Ok(())
//     }
//     fn get_response(&self) -> Option<Responses> {
//         Some(Responses::Program(String::from("Programm wurde kopiert")))
//     }
// }

// impl Config for ConfigCopyProgramToSys {
//     fn get_body(&self) -> String {
//         String::from("")
//     }
//     fn get_path(&self) -> String {
//         String::from("")
//     }
// }

// impl Config for ConfigUpdateProgramSource {
//     fn get_body(&self) -> String {
//         self.body.clone()
//     }
//     fn get_path(&self) -> String {
//         format!(
//             "/sap/bc/adt/programs/programs/{}/source/main?lockHandle={}",
//             self.prog_name,
//             self.lock_handle.as_ref().unwrap()
//         )
//     }
// }

// impl ConfigUpdateProgramSource {
//     pub fn new(prog_name: &str, source: &str) -> Self {
//         ConfigUpdateProgramSource {
//             body: String::from(source),
//             prog_name: String::from(prog_name),
//             lock_handle: None,
//         }
//     }
// }
// impl LockHandle for ConfigUpdateProgramSource {
//     fn get_lock_handle_path(&self) -> String {
//         format!(
//             "/sap/bc/adt/programs/programs/{}?_action=LOCK&accessMode=MODIFY",
//             self.prog_name
//         )
//     }
//     fn get_unlock_path(&self) -> Option<String> {
//         Some(format!(
//             "/sap/bc/adt/programs/programs/{}?_action=UNLOCK&lockHandle={}",
//             self.prog_name,
//             self.lock_handle.as_ref()?
//         ))
//     }
// }
// #[async_trait]
// impl Sendable for ConfigUpdateProgramSource {
//     async fn send_with(&mut self, client: &mut SAPClient) -> Result<(), AdtError> {
//         client.set_stateful(true);

//         let lock_handle_res = client.lock(self).await;

//         let xml = lock_handle_res.text().await.unwrap();

//         // println!("{:?}", &xml);
//         let lock_handle: LockHandleResponse = quick_xml::de::from_str(&xml).unwrap();
//         // println!("{:?}", lock_handle);
//         self.lock_handle = Some(lock_handle.values.DATA.LOCK_HANDLE);
//         let res = client.put(self).await;

//         client.unlock(self).await;

//         // println!("{}", res.status());
//         // println!("{}", res.text().await.unwrap());
//         // self.source = res.text().await.ok();
//         Ok(())
//     }
//     fn get_response(&self) -> Option<Responses> {
//         Some(Responses::Program(String::from("")))
//     }
// }
// impl SendableConfig for ConfigUpdateProgramSource {}
// #[derive(Debug)]
// pub struct ConfigGetProgramSource {
//     body: String,
//     path: String,
//     source: Option<String>,
// }

// impl ConfigGetProgramSource {
//     pub fn new(prog_name: &str) -> ConfigGetProgramSource {
//         ConfigGetProgramSource {
//             body: String::from(""),
//             path: format!("/sap/bc/adt/programs/programs/{}/source/main", prog_name),
//             source: None,
//         }
//     }
//     pub fn get_source(&self) -> Option<String> {
//         self.source.clone()
//     }
// }

// impl Config for ConfigGetProgramSource {
//     fn get_body(&self) -> String {
//         self.body.clone()
//     }
//     fn get_path(&self) -> String {
//         self.path.clone()
//     }
// }
// #[async_trait]
// impl Sendable for ConfigGetProgramSource {
//     async fn send_with(&mut self, client: &mut SAPClient) -> Result<(), AdtError> {
//         let res = client.get(self).await;

//         // println!("{}", res.status());

//         self.source = res.text().await.ok();
//         Ok(())
//     }
//     fn get_response(&self) -> Option<Responses> {
//         Some(Responses::Program(String::from("")))
//     }
// }
// impl SendableConfig for ConfigGetProgramSource {}

// pub struct ProgramResponse {}

// pub struct ProgramError {}

// impl ConfigCreateProgram {
//     pub fn new(
//         prog_name: &str,
//         package_name: Option<&str>,
//         transport_request: Option<&str>,
//     ) -> Self {
//         ConfigCreateProgram {
//             //ITKK901409
//             path: match transport_request {
//                 Some(t) => format!("/sap/bc/adt/programs/programs?corrNr={0}", t),
//                 None => String::from("/sap/bc/adt/programs/programs"),
//             },

//             body: format!(
//                 r#"<?xml version="1.0" encoding="UTF-8"?>
//                         <program:abapProgram xmlns:program="http://www.sap.com/adt/programs/programs" xmlns:adtcore="http://www.sap.com/adt/core"
//                             adtcore:description="Von Programm erstellt" adtcore:language="DE" adtcore:name="{program_name}" adtcore:type="PROG/P" adtcore:masterLanguage="DE"
//                             adtcore:masterSystem="ITK" adtcore:responsible="PFRANK">

//                             <adtcore:packageRef adtcore:name="{package_name}"/>
//                     </program:abapProgram>"#,
//                 program_name = prog_name,
//                 package_name = package_name.unwrap_or("$TMP")
//             ),
//             text: None,
//             prog_name: prog_name.to_string(),
//         }
//     }
// }

// impl ConfigDeleteProgram {
//     pub fn new(prog_name: &str, transport_request: &str) -> Self {
//         ConfigDeleteProgram {
//             path: String::new(),
//             body: String::new(),
//             prog_name: prog_name.to_string(),
//             transport_request: transport_request.to_string(),
//             lock_handle: None,
//         }
//     }
// }

// impl ConfigCopyProgram {
//     pub fn new(
//         prog_name: &str,
//         package_name: &str,
//         source_prog_name: &str,
//         transport_request: &str,
//     ) -> Self {
//         ConfigCopyProgram {
//             path: format!("/sap/bc/adt/programs/programs?corrNr={}", transport_request),
//             body: format!(
//                 r#"<?xml version="1.0" encoding="UTF-8"?><program:abapProgram xmlns:program="http://www.sap.com/adt/programs/programs" xmlns:abapsource="http://www.sap.com/adt/abapsource" xmlns:adtcore="http://www.sap.com/adt/core" adtcore:language="DE" adtcore:name="{prog_name}" adtcore:type="PROG/P" adtcore:masterLanguage="DE" adtcore:masterSystem="ITK" adtcore:responsible="PFRANK">

//             <adtcore:packageRef adtcore:name="{package_name}"/>

//             <abapsource:template abapsource:name="{source_prog}"/>

//           </program:abapProgram>"#,
//                 prog_name = prog_name,
//                 package_name = package_name,
//                 source_prog = source_prog_name
//             ),
//             prog_name: prog_name.to_string(),
//         }
//     }
// }
// #[async_trait]
// impl SendWith for ConfigCreateProgram {
//     async fn send_with(&self, client: &mut super::SAPClient) -> reqwest::Response {
//         client.send(self).await
//     }
// }

// impl Config for ConfigDeleteProgram {
//     fn get_body(&self) -> String {
//         self.body.clone()
//     }
//     fn get_path(&self) -> String {
//         format!(
//             "/sap/bc/adt/programs/programs/{0}?corrNr={1}&lockHandle={2}",
//             self.prog_name,
//             self.transport_request,
//             self.lock_handle.as_ref().unwrap()
//         )
//     }

// async fn send_with(
//     &mut self,
//     client: &mut super::SAPClient,
// ) -> Result<ProgramResponse, ProgramError> {
//     let lock_handle_res = client.send(self).await;

//     let xml = lock_handle_res.text().await.unwrap();
//     // println!("{:?}", &xml);
//     let lock_handle: LockHandleResponse = quick_xml::de::from_str(&xml).unwrap();

//     self.lock_handle = Some(lock_handle.values.DATA.LOCK_HANDLE);

//     client.delete(self).await;

//     Ok(ProgramResponse {})
// }
// }
// #[async_trait]
// impl Sendable for ConfigDeleteProgram {
//     async fn send_with(&mut self, client: &mut SAPClient) -> Result<(), AdtError> {
//         client.set_stateful(true);

//         let lock_handle_res = client.lock(self).await;

//         let xml = lock_handle_res.text().await.unwrap();
//         // println!("{:?}", &xml);
//         let lock_handle: LockHandleResponse = quick_xml::de::from_str(&xml).unwrap();

//         self.lock_handle = Some(lock_handle.values.DATA.LOCK_HANDLE);

//         let res = client.delete(self).await;
//         // println!("{}", res.text().await.unwrap());
//         Ok(())
//     }
//     fn get_response(&self) -> Option<Responses> {
//         Some(Responses::Program(String::from("")))
//     }
// }
// #[async_trait]
// impl Sendable<ProgramResponse, ProgramError> for ConfigDeleteProgram {
//     async fn send_with(&mut self, client: &mut SAPClient) -> Result<ProgramResponse, ProgramError> {
//         let lock_handle_res = client.send(self).await;

//         let xml = lock_handle_res.text().await.unwrap();
//         // println!("{:?}", &xml);
//         let lock_handle: LockHandleResponse = quick_xml::de::from_str(&xml).unwrap();

//         self.lock_handle = Some(lock_handle.values.DATA.LOCK_HANDLE);

//         client.delete(self).await;

//         Ok(ProgramResponse {})
//     }
// // }
// #[async_trait]
// impl Sendable<ProgramResponse, ProgramError> for ConfigCreateProgram {
//     async fn send_with(&mut self, client: &mut SAPClient) -> Result<ProgramResponse, ProgramError> {
//         client.send(self).await;
//         Ok(ProgramResponse {})
//     }
// }
// #[async_trait]
// impl Sendable<ProgramResponse, ProgramError> for ConfigCopyProgram {
//     async fn send_with(&mut self, client: &mut SAPClient) -> Result<ProgramResponse, ProgramError> {
//         client.send(self).await;
//         Ok(ProgramResponse {})
//     }
// }

// impl Config for ConfigCreateProgram {
//     fn get_body(&self) -> String {
//         self.body.clone()
//     }
//     fn get_path(&self) -> String {
//         self.path.clone()
//     }
// }
// #[async_trait]
// impl Sendable for ConfigCreateProgram {
//     async fn send_with(&mut self, client: &mut SAPClient) -> Result<(), AdtError> {
//         let res = client.send(self).await;

//         if res.status() == 500 {
//             return Err(AdtError::new("Program already exists"));
//         }

//         self.text = match res.text().await {
//             Ok(_text) => Some(String::from("Program created")),
//             Err(_e) => return Err(AdtError::new("Program couldnt be created")),
//         };
//         // println!("{}", self.text.as_ref().unwrap());
//         Ok(())
//     }

//     fn get_response(&self) -> Option<Responses> {
//         Some(Responses::Program(self.text.clone()?))
//     }
// }
// impl SendableConfig for ConfigCreateProgram {}
// impl SendableConfig for ConfigCopyProgram {}
// impl SendableConfig for ConfigDeleteProgram {}

// impl Config for ConfigCopyProgram {
//     fn get_body(&self) -> String {
//         self.body.clone()
//     }
//     fn get_path(&self) -> String {
//         self.path.clone()
//     }
// }
// #[async_trait]
// impl Sendable for ConfigCopyProgram {
//     async fn send_with(&mut self, client: &mut SAPClient) -> Result<(), AdtError> {
//         let res = client.send(self).await;
//         // println!("{}", res.status());
//         // println!("{}", res.text().await.unwrap());
//         Ok(())
//     }
//     fn get_response(&self) -> Option<Responses> {
//         Some(Responses::Program(String::from("Program copied")))
//     }
// }

// impl LockHandle for ConfigDeleteProgram {
//     fn get_lock_handle_path(&self) -> String {
//         format!(
//             "/sap/bc/adt/programs/programs/{0}?_action=LOCK&accessMode=MODIFY",
//             self.prog_name
//         )
//     }
//     fn get_unlock_path(&self) -> Option<String> {
//         Some(format!(
//             "/sap/bc/adt/programs/programs/{}?_action=UNLOCK&lockHandle={}",
//             self.prog_name,
//             self.lock_handle.as_ref()?
//         ))
//     }
// }
#[derive(Debug)]
pub struct ConfigExecuteProgram {
    body: String,
    path: String,
}

impl ConfigExecuteProgram {
    fn new(prog: &AbapProg) -> Self {
        ConfigExecuteProgram {
            path: String::from("/sap/bc/soap/rfc"),
            body: format!(
                r#"<SOAP-ENV:Envelope xmlns:SOAP-ENV="http://schemas.xmlsoap.org/soap/envelope/" xmlns:x="urn:sap-com:document:sap:rfc:functions">
    <SOAP-ENV:Header></SOAP-ENV:Header>
    <SOAP-ENV:Body>
        <x:RFC_ABAP_INSTALL_AND_RUN>
            <PROGRAM>{}</PROGRAM>
        </x:RFC_ABAP_INSTALL_AND_RUN>
    </SOAP-ENV:Body>
</SOAP-ENV:Envelope>"#,
                prog.into_xml()
            ),
        }
    }
}
impl SendableConfig for ConfigExecuteProgram {}
#[async_trait]
impl Sendable for ConfigExecuteProgram {
    async fn send_with(&mut self, client: &mut SAPClient) -> Result<(), AdtError> {
        // let res = client.send(self).await;
        // println!("{}", res.status());
        // println!("{}", res.text().await.unwrap());
        Ok(())
    }
    fn get_response(&self) -> Option<Responses> {
        Some(Responses::Program(String::from("Program executed")))
    }
}
impl Config for ConfigExecuteProgram {
    fn get_body(&self) -> String {
        self.body.clone()
    }
    fn get_path(&self) -> String {
        self.path.clone()
    }
}
// #[derive(Debug)]
// pub struct ConfigCreateTable {
//     body: String,
//     path: String,
//     tab_name: String,
//     tab_descr: String,
// }
// impl ConfigCreateTable {
//     pub fn new(tab_name: &str, tab_descr: &str) -> Self {
//         ConfigCreateTable {
//             body: String::from(""),
//             path: String::from(""),
//             tab_name: String::from(tab_name).to_uppercase(),
//             tab_descr: String::from(tab_descr),
//         }
//     }
// }
// impl SendableConfig for ConfigCreateTable {}
// #[async_trait]
// impl Sendable for ConfigCreateTable {
//     async fn send_with(&mut self, client: &mut SAPClient) -> Result<(), AdtError> {
//         let mut prog = AbapProg::new("zghdrngjslvfnrfwwfkdfhjfke");
//         prog.add_line("DATA:")
//             .add_line("lt_fields TYPE TABLE OF dd03p,")
//             .add_line("wa_field TYPE dd03p,")
//             .add_line("table_header TYPE dd02v,")
//             .add_line("techn_set TYPE dd09v.")
//             .add_line(&format!("table_header-tabname = '{}'.", self.tab_name))
//             .add_line(&format!("table_header-ddtext = '{}'.", self.tab_descr))
//             .add_line("table_header-ddlanguage = sy-langu.")
//             .add_line("table_header-tabclass = 'TRANSP'.")
//             .add_line("table_header-as4user = sy-uname.")
//             .add_line("table_header-contflag = 'A'.")
//             .add_line("table_header-mainflag = 'X'.")
//             .add_line(&format!("techn_set-tabname = '{}'.", self.tab_name))
//             .add_line("techn_set-tabkat = 0.")
//             .add_line("techn_set-tabart = 'APPL1'.")
//             .add_line("techn_set-bufallow = 'X'.")
//             .add_line("techn_set-pufferung = 'X'.")
//             .add_line(&format!("wa_field-tabname = '{}'.", self.tab_name))
//             .add_line("wa_field-ddlanguage = sy-langu.")
//             .add_line("wa_field-notnull = 'X'.")
//             .add_line("wa_field-keyflag = 'X'.")
//             .add_line("wa_field-fieldname = 'ID'.")
//             .add_line("wa_field-position = '1'.")
//             .add_line("wa_field-rollname = 'CHAR10'.")
//             .add_line("APPEND wa_field TO lt_fields.")
//             .add_function_call("DDIF_TABL_PUT")
//             .exporting("name", &format!("'{}'", self.tab_name))
//             .tables("dd03p_tab", "lt_fields")
//             .dot()
//             .add_function_call("DDIF_TABL_PUT")
//             .exporting("dd02v_wa", "table_header")
//             .exporting("dd09l_wa", "techn_set")
//             .tables("dd03p_tab", "lt_fields")
//             .dot()
//             // .add_line("CALL FUNCTION 'DDIF_TABL_PUT'")
//             // .add_line("EXPORTING")
//             // .add_line(&format!("name = '{}'", self.tab_name))
//             // .add_line("dd02v_wa = table_header")
//             // .add_line("dd09l_wa = techn_set")
//             // .add_line("TABLES")
//             // .add_line("dd03p_tab = lt_fields.")
//             // .add_line("  EXCEPTIONS")
//             // .add_line("    tabl_not_found    = 1")
//             // .add_line("    name_inconsistent = 2")
//             // .add_line("    tabl_inconsistent = 3")
//             // .add_line("    put_failure       = 4")
//             // .add_line("    put_refused       = 5")
//             // .add_line("    OTHERS            = 6.")
//             .add_line("CALL FUNCTION 'DDIF_TABL_ACTIVATE'")
//             .add_line("EXPORTING")
//             .add_line(&format!("name = '{}'.", self.tab_name));
//         // .add_line("  EXCEPTIONS")
//         // .add_line("NOT_FOUND   = 1")
//         // .add_line("")
//         // .add_line("   PUT_FAILURE = 2")
//         // .add_line("    OTHERS      = 3.");

//         ConfigExecuteProgram::new(&prog).send_with(client).await;

//         Ok(())
//     }
//     fn get_response(&self) -> Option<Responses> {
//         Some(Responses::Program(String::from("Tabelle erstellt")))
//     }
// }
// impl Config for ConfigCreateTable {
//     fn get_body(&self) -> String {
//         self.body.clone()
//     }
//     fn get_path(&self) -> String {
//         self.path.clone()
//     }
// }
#[derive(Debug)]
pub struct ConfigGetTableDetails {
    path: String,
    body: String,
    data: Option<SoapResponse>,
}
impl Config for ConfigGetTableDetails {
    fn get_body(&self) -> String {
        self.body.clone()
    }
    fn get_path(&self) -> String {
        self.path.clone()
    }
}
impl ConfigGetTableDetails {
    pub fn new(tab_name: &str) -> Self {
        ConfigGetTableDetails {
            data: None,
            path: String::from("/sap/bc/soap/rfc"),
            body: format!(
                r#"<SOAP-ENV:Envelope xmlns:SOAP-ENV="http://schemas.xmlsoap.org/soap/envelope/" xmlns:x="urn:sap-com:document:sap:rfc:functions">
            <SOAP-ENV:Header></SOAP-ENV:Header>
            <SOAP-ENV:Body>
                <x:BDL_DDIF_TABL_GET>
                    <NAME>{}</NAME>
        
                    <DD03P_TAB/>
                </x:BDL_DDIF_TABL_GET>
            </SOAP-ENV:Body>
        </SOAP-ENV:Envelope>"#,
                tab_name
            ),
        }
    }
    fn get_data(&self) -> Option<&SoapResponse> {
        self.data.as_ref()
    }
}
impl SendableConfig for ConfigGetTableDetails {}
#[async_trait]
impl Sendable for ConfigGetTableDetails {
    async fn send_with(&mut self, client: &mut SAPClient) -> Result<(), AdtError> {
        // let res = client.send(self).await;
        // let text = &res.text().await.unwrap();
        // println!("{:?}", &text);
        // let data: SoapResponse = quick_xml::de::from_str(text).unwrap();
        // self.data = Some(data);
        // println!("{:?}", data);
        Ok(())
    }
    fn get_response(&self) -> Option<Responses> {
        Some(Responses::Class("".to_string()))
    }
}
#[derive(Debug)]
struct ConfigPutTableDetails {
    body: String,
    path: String,
}
impl Config for ConfigPutTableDetails {
    fn get_body(&self) -> String {
        self.body.clone()
    }
    fn get_path(&self) -> String {
        self.path.clone()
    }
}
impl ConfigPutTableDetails {
    pub fn new(
        tab_name: &str,
        dd02v: &Dd02v,
        dd09l: &Dd09l,
        dd03p: &DD03P_TAB,
        package: &str,
        reqnum: &str,
    ) -> Self {
        ConfigPutTableDetails {
            path: String::from("/sap/bc/soap/rfc"),
            body: format!(
                r#"<SOAP-ENV:Envelope xmlns:SOAP-ENV="http://schemas.xmlsoap.org/soap/envelope/" xmlns:x="urn:sap-com:document:sap:rfc:functions">
            <SOAP-ENV:Header></SOAP-ENV:Header>
            <SOAP-ENV:Body>
                <x:ITSAMCG_DDIF_TABL_PUT>
                <REQNUM>{}</REQNUM>
                <SRCSYSTEM>ITK</SRCSYSTEM>
                <DEVCLASS>{}</DEVCLASS>
                <TYPENAME>TABL</TYPENAME>
                 <NAME>{}</NAME>
                 {}{}{}
        </x:ITSAMCG_DDIF_TABL_PUT>
        </SOAP-ENV:Body>
    </SOAP-ENV:Envelope>"#,
                reqnum,
                package,
                tab_name,
                quick_xml::se::to_string(dd02v).unwrap(),
                quick_xml::se::to_string(dd09l).unwrap(),
                quick_xml::se::to_string(dd03p).unwrap()
            ), // body: format!(
               //     r#"<SOAP-ENV:Envelope xmlns:SOAP-ENV="http://schemas.xmlsoap.org/soap/envelope/" xmlns:x="urn:sap-com:document:sap:rfc:functions">
               // <SOAP-ENV:Header></SOAP-ENV:Header>
               // <SOAP-ENV:Body>
               //     <x:ITSAMCG_DDIF_TABL_PUT>"#,
               //     body
               // ),S
        }
    }
}
impl SendableConfig for ConfigPutTableDetails {}
#[async_trait]
impl Sendable for ConfigPutTableDetails {
    async fn send_with(&mut self, client: &mut SAPClient) -> Result<(), AdtError> {
        // // println!("{}", self.body);
        // let res = client.send(self).await;
        // let text = &res.text().await.unwrap();

        // println!("{:?}", &text);
        // let data: SoapResponse = quick_xml::de::from_str(text).unwrap();

        // println!("{:?}", data);
        Ok(())
    }
    fn get_response(&self) -> Option<Responses> {
        Some(Responses::Class("".to_string()))
    }
}
#[derive(Debug)]
pub struct ConfigCopyDatabaseTable {
    body: String,
    path: String,
    tab_name: String,
    dest: Destination,
    package: String,
    reqnum: String,
}
impl Config for ConfigCopyDatabaseTable {
    fn get_body(&self) -> String {
        self.body.clone()
    }
    fn get_path(&self) -> String {
        self.path.clone()
    }
}
impl ConfigCopyDatabaseTable {
    pub fn new(tab_name: &str, dest: &Destination, package: &str, reqnum: &str) -> Self {
        ConfigCopyDatabaseTable {
            path: String::from("/sap/bc/soap/rfc"),
            tab_name: tab_name.to_string(),
            dest: dest.clone(),
            body: "".to_string(),
            package: package.to_string(),
            reqnum: reqnum.to_string(),
        }
    }
}

#[async_trait]
impl Sendable for ConfigCopyDatabaseTable {
    async fn send_with(&mut self, client: &mut SAPClient) -> Result<(), AdtError> {
        // let res = client.send(self).await;
        // let text = &res.text().await.unwrap();
        let mut get_table_details = ConfigGetTableDetails::new(&self.tab_name);
        get_table_details.send_with(client).await.unwrap();

        let details = get_table_details.get_data().unwrap();

        client.set_destination(&self.dest);
        let res = &details.body.response;
        // let body = &quick_xml::se::to_string(&details).unwrap();
        // println!("{}", &body);
        let mut put_table_details = ConfigPutTableDetails::new(
            &self.tab_name,
            &res.dd02v,
            &res.dd09l,
            &res.fields,
            &self.package,
            &self.reqnum,
        );

        put_table_details.send_with(client).await;
        // println!("{:?}", &text);
        // let data: SoapResponse = quick_xml::de::from_str(text).unwrap();

        // println!("{:?}", data);
        Ok(())
    }
    fn get_response(&self) -> Option<Responses> {
        Some(Responses::Class("".to_string()))
    }
}
impl SendableConfig for ConfigCopyDatabaseTable {}
