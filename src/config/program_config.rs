use std::fmt::Error;

use crate::net::Destination;

use super::{
    AdtError, AdtResponse, Config, LockHandle, Responses, SAPClient, Sendable, SendableConfig,
};
// use crate::config::Sendable;
use async_trait::async_trait;
use serde::Deserialize;
// pub trait Config {
//     fn get_path(&self) -> &String;
//     fn get_body(&self) -> &String;
// }
#[derive(Debug, Deserialize)]
#[serde(rename = "DATA")]
struct LockHandleData {
    LOCK_HANDLE: String,
    CORRNR: String,
    CORRUSER: String,
    CORRTEXT: String,
}

#[derive(Debug, Deserialize)]
struct LockHandleValues {
    DATA: LockHandleData,
}
#[derive(Debug, Deserialize)]
#[serde(rename = "asx:abap")]
struct LockHandleResponse {
    // #[serde(rename = "asx:values")]
    values: LockHandleValues,
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

#[derive(Debug)]
pub struct ConfigCreateProgram {
    body: String,
    path: String,
    prog_name: String,
    text: Option<String>,
}
#[derive(Debug)]
pub struct ConfigDeleteProgram {
    body: String,
    path: String,
    prog_name: String,
    lock_handle: Option<String>,
    transport_request: String,
}
#[derive(Debug)]
pub struct ConfigCopyProgram {
    body: String,
    path: String,
    prog_name: String,
}
#[derive(Debug)]
pub struct ConfigUpdateProgramSource {
    body: String,
    prog_name: String,
    lock_handle: Option<String>,
}
#[derive(Debug)]
pub struct ConfigCopyProgramToSys {
    prog_name: String,
}

impl ConfigCopyProgramToSys {
    pub fn new(prog_name: &str) -> Self {
        ConfigCopyProgramToSys {
            prog_name: String::from(prog_name),
        }
    }
}

#[async_trait]
impl Sendable for ConfigCopyProgramToSys {
    async fn send_with(&mut self, client: &mut SAPClient) -> Result<(), AdtError> {
        let mut conf_get_source = ConfigGetProgramSource::new(&self.prog_name);

        conf_get_source.send_with(client).await?;

        client.set_destination(&Destination {
            host: String::from("https://hamerpiea.zalaris.de"),
            port: 443,
            sys_id: String::from("IEA"),
            uname: String::from("PFRANK"),
            passwd: String::from("Start1234$"),
            mandt: String::from("200"),
            lang: String::from("DE"),
        });

        println!("{}", &conf_get_source.get_source().unwrap());
        client.set_stateful(true);
        client.clear_session();
        ConfigCreateProgram::new(&self.prog_name, None, None)
            .send_with(client)
            .await?;

        ConfigUpdateProgramSource::new(&self.prog_name, &conf_get_source.get_source().unwrap())
            .send_with(client)
            .await?;

        Ok(())
    }
    fn get_response(&self) -> Option<Responses> {
        Some(Responses::Program(String::from("Programm wurde kopiert")))
    }
}

impl Config for ConfigCopyProgramToSys {
    fn get_body(&self) -> String {
        String::from("")
    }
    fn get_path(&self) -> String {
        String::from("")
    }
}

impl Config for ConfigUpdateProgramSource {
    fn get_body(&self) -> String {
        self.body.clone()
    }
    fn get_path(&self) -> String {
        format!(
            "/sap/bc/adt/programs/programs/{}/source/main?lockHandle={}",
            self.prog_name,
            self.lock_handle.as_ref().unwrap()
        )
    }
}

impl ConfigUpdateProgramSource {
    pub fn new(prog_name: &str, source: &str) -> Self {
        ConfigUpdateProgramSource {
            body: String::from(source),
            prog_name: String::from(prog_name),
            lock_handle: None,
        }
    }
}
impl LockHandle for ConfigUpdateProgramSource {
    fn get_lock_handle_path(&self) -> String {
        format!(
            "/sap/bc/adt/programs/programs/{}?_action=LOCK&accessMode=MODIFY",
            self.prog_name
        )
    }
    fn get_unlock_path(&self) -> Option<String> {
        Some(format!(
            "/sap/bc/adt/programs/programs/{}?_action=UNLOCK&lockHandle={}",
            self.prog_name,
            self.lock_handle.as_ref()?
        ))
    }
}
#[async_trait]
impl Sendable for ConfigUpdateProgramSource {
    async fn send_with(&mut self, client: &mut SAPClient) -> Result<(), AdtError> {
        client.set_stateful(true);

        let lock_handle_res = client.lock(self).await;

        let xml = lock_handle_res.text().await.unwrap();

        println!("{:?}", &xml);
        let lock_handle: LockHandleResponse = quick_xml::de::from_str(&xml).unwrap();
        println!("{:?}", lock_handle);
        self.lock_handle = Some(lock_handle.values.DATA.LOCK_HANDLE);
        let res = client.put(self).await;

        client.unlock(self).await;

        println!("{}", res.status());
        println!("{}", res.text().await.unwrap());
        // self.source = res.text().await.ok();
        Ok(())
    }
    fn get_response(&self) -> Option<Responses> {
        Some(Responses::Program(String::from("")))
    }
}
impl SendableConfig for ConfigUpdateProgramSource {}
#[derive(Debug)]
pub struct ConfigGetProgramSource {
    body: String,
    path: String,
    source: Option<String>,
}

impl ConfigGetProgramSource {
    pub fn new(prog_name: &str) -> ConfigGetProgramSource {
        ConfigGetProgramSource {
            body: String::from(""),
            path: format!("/sap/bc/adt/programs/programs/{}/source/main", prog_name),
            source: None,
        }
    }
    pub fn get_source(&self) -> Option<String> {
        self.source.clone()
    }
}

impl Config for ConfigGetProgramSource {
    fn get_body(&self) -> String {
        self.body.clone()
    }
    fn get_path(&self) -> String {
        self.path.clone()
    }
}
#[async_trait]
impl Sendable for ConfigGetProgramSource {
    async fn send_with(&mut self, client: &mut SAPClient) -> Result<(), AdtError> {
        let res = client.get(self).await;

        println!("{}", res.status());

        self.source = res.text().await.ok();
        Ok(())
    }
    fn get_response(&self) -> Option<Responses> {
        Some(Responses::Program(String::from("")))
    }
}
impl SendableConfig for ConfigGetProgramSource {}

pub struct ProgramResponse {}

pub struct ProgramError {}

impl ConfigCreateProgram {
    pub fn new(
        prog_name: &str,
        package_name: Option<&str>,
        transport_request: Option<&str>,
    ) -> Self {
        ConfigCreateProgram {
            //ITKK901409
            path: match transport_request {
                Some(t) => format!("/sap/bc/adt/programs/programs?corrNr={0}", t),
                None => String::from("/sap/bc/adt/programs/programs"),
            },

            body: format!(
                r#"<?xml version="1.0" encoding="UTF-8"?>
                        <program:abapProgram xmlns:program="http://www.sap.com/adt/programs/programs" xmlns:adtcore="http://www.sap.com/adt/core" 
                            adtcore:description="Von Programm erstellt" adtcore:language="DE" adtcore:name="{program_name}" adtcore:type="PROG/P" adtcore:masterLanguage="DE" 
                            adtcore:masterSystem="ITK" adtcore:responsible="PFRANK">  
                        
                            <adtcore:packageRef adtcore:name="{package_name}"/>
                    </program:abapProgram>"#,
                program_name = prog_name,
                package_name = package_name.unwrap_or("$TMP")
            ),
            text: None,
            prog_name: prog_name.to_string(),
        }
    }
}

impl ConfigDeleteProgram {
    pub fn new(prog_name: &str, transport_request: &str) -> Self {
        ConfigDeleteProgram {
            path: String::new(),
            body: String::new(),
            prog_name: prog_name.to_string(),
            transport_request: transport_request.to_string(),
            lock_handle: None,
        }
    }
}

impl ConfigCopyProgram {
    pub fn new(
        prog_name: &str,
        package_name: &str,
        source_prog_name: &str,
        transport_request: &str,
    ) -> Self {
        ConfigCopyProgram {
            path: format!("/sap/bc/adt/programs/programs?corrNr={}", transport_request),
            body: format!(
                r#"<?xml version="1.0" encoding="UTF-8"?><program:abapProgram xmlns:program="http://www.sap.com/adt/programs/programs" xmlns:abapsource="http://www.sap.com/adt/abapsource" xmlns:adtcore="http://www.sap.com/adt/core" adtcore:language="DE" adtcore:name="{prog_name}" adtcore:type="PROG/P" adtcore:masterLanguage="DE" adtcore:masterSystem="ITK" adtcore:responsible="PFRANK">
    
            <adtcore:packageRef adtcore:name="{package_name}"/>
              
            <abapsource:template abapsource:name="{source_prog}"/>
            
          </program:abapProgram>"#,
                prog_name = prog_name,
                package_name = package_name,
                source_prog = source_prog_name
            ),
            prog_name: prog_name.to_string(),
        }
    }
}
// #[async_trait]
// impl SendWith for ConfigCreateProgram {
//     async fn send_with(&self, client: &mut super::SAPClient) -> reqwest::Response {
//         client.send(self).await
//     }
// }

impl Config for ConfigDeleteProgram {
    fn get_body(&self) -> String {
        self.body.clone()
    }
    fn get_path(&self) -> String {
        format!(
            "/sap/bc/adt/programs/programs/{0}?corrNr={1}&lockHandle={2}",
            self.prog_name,
            self.transport_request,
            self.lock_handle.as_ref().unwrap()
        )
    }

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
}
#[async_trait]
impl Sendable for ConfigDeleteProgram {
    async fn send_with(&mut self, client: &mut SAPClient) -> Result<(), AdtError> {
        client.set_stateful(true);

        let lock_handle_res = client.lock(self).await;

        let xml = lock_handle_res.text().await.unwrap();
        // println!("{:?}", &xml);
        let lock_handle: LockHandleResponse = quick_xml::de::from_str(&xml).unwrap();

        self.lock_handle = Some(lock_handle.values.DATA.LOCK_HANDLE);

        let res = client.delete(self).await;
        // println!("{}", res.text().await.unwrap());
        Ok(())
    }
    fn get_response(&self) -> Option<Responses> {
        Some(Responses::Program(String::from("")))
    }
}
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

impl Config for ConfigCreateProgram {
    fn get_body(&self) -> String {
        self.body.clone()
    }
    fn get_path(&self) -> String {
        self.path.clone()
    }
}
#[async_trait]
impl Sendable for ConfigCreateProgram {
    async fn send_with(&mut self, client: &mut SAPClient) -> Result<(), AdtError> {
        let res = client.send(self).await;

        if res.status() == 500 {
            return Err(AdtError::new("Program already exists"));
        }

        self.text = match res.text().await {
            Ok(_text) => Some(String::from("Program created")),
            Err(_e) => return Err(AdtError::new("Program couldnt be created")),
        };
        println!("{}", self.text.as_ref().unwrap());
        Ok(())
    }

    fn get_response(&self) -> Option<Responses> {
        Some(Responses::Program(self.text.clone()?))
    }
}
impl SendableConfig for ConfigCreateProgram {}
impl SendableConfig for ConfigCopyProgram {}
impl SendableConfig for ConfigDeleteProgram {}

impl Config for ConfigCopyProgram {
    fn get_body(&self) -> String {
        self.body.clone()
    }
    fn get_path(&self) -> String {
        self.path.clone()
    }
}
#[async_trait]
impl Sendable for ConfigCopyProgram {
    async fn send_with(&mut self, client: &mut SAPClient) -> Result<(), AdtError> {
        let res = client.send(self).await;
        println!("{}", res.status());
        println!("{}", res.text().await.unwrap());
        Ok(())
    }
    fn get_response(&self) -> Option<Responses> {
        Some(Responses::Program(String::from("Program copied")))
    }
}

impl LockHandle for ConfigDeleteProgram {
    fn get_lock_handle_path(&self) -> String {
        format!(
            "/sap/bc/adt/programs/programs/{0}?_action=LOCK&accessMode=MODIFY",
            self.prog_name
        )
    }
    fn get_unlock_path(&self) -> Option<String> {
        Some(format!(
            "/sap/bc/adt/programs/programs/{}?_action=UNLOCK&lockHandle={}",
            self.prog_name,
            self.lock_handle.as_ref()?
        ))
    }
}
