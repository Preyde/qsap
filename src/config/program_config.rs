use super::{AdtError, AdtResponse, Config, LockHandle, Responses, SAPClient};
use crate::config::Sendable;
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
impl AdtResponse for ProgramResponse {
    fn get_data(self) -> Responses {
        Responses::Program(())
    }
}
impl AdtError for ProgramError {}
// pub trait DeleteConfig<T>
// where
//     T: Config,
// {
// }

// pub struct ProgramConfig {
//     body: String,
//     path: String,
//     prog_name: String,
// }

// trait AdtObject {}
// struct AdtProgram {}

// // impl AdtProgram {
// //     fn config_create() -> Config {}
// // }

// struct ConfigCreateProgram {}

// impl ConfigCreateProgram {}
// struct ConfigDeleteProgram {}
// struct ConfigCopyProgram {}

// struct ConfigDelete {}

// // impl ProgramConfig {
// //     pub fn new(prog_name: &str, package_name: &str, transport_request: &str) -> Self {
// //         ProgramConfig {
// //             //ITKK901409
// //             path: format!(
// //                 "/sap/bc/adt/programs/programs?corrNr={0}&sap-client=300",
// //                 transport_request
// //             ),
// //             body: format!(
// //                 r#"<?xml version="1.0" encoding="UTF-8"?>
// //                         <program:abapProgram xmlns:program="http://www.sap.com/adt/programs/programs" xmlns:adtcore="http://www.sap.com/adt/core"
// //                             adtcore:description="Von Programm erstellt" adtcore:language="DE" adtcore:name="{program_name}" adtcore:type="PROG/P" adtcore:masterLanguage="DE"
// //                             adtcore:masterSystem="ITK" adtcore:responsible="PFRANK">

// //                             <adtcore:packageRef adtcore:name="{package_name}"/>
// //                     </program:abapProgram>"#,
// //                 program_name = prog_name,
// //                 package_name = package_name
// //             ),
// //             prog_name: prog_name.to_string(),
// //         }
// //     }
// //     pub fn delete(prog_name: &str, lock_handle: &str, transport_request: &str) -> Self {
// //         ProgramConfig {
// //             path: format!(
// //                 "/sap/bc/adt/programs/programs/{0}?corrNr={1}&lockHandle={2}&sap-client=300",
// //                 prog_name, transport_request, lock_handle
// //             ),
// //             body: String::new(),
// //             prog_name: prog_name.to_string(),
// //         }
// //     }
// //     pub fn copy(
// //         prog_name: &str,
// //         package_name: &str,
// //         source_prog_name: &str,
// //         transport_request: &str,
// //     ) -> Self {
// //         ProgramConfig {
// //             path: format!(
// //                 "/sap/bc/adt/programs/programs?corrNr={}&sap-client=300",
// //                 transport_request
// //             ),
// //             body: format!(
// //                 r#"<?xml version="1.0" encoding="UTF-8"?><program:abapProgram xmlns:program="http://www.sap.com/adt/programs/programs" xmlns:abapsource="http://www.sap.com/adt/abapsource" xmlns:adtcore="http://www.sap.com/adt/core" adtcore:language="DE" adtcore:name="{prog_name}" adtcore:type="PROG/P" adtcore:masterLanguage="DE" adtcore:masterSystem="ITK" adtcore:responsible="PFRANK">

// //             <adtcore:packageRef adtcore:name="{package_name}"/>

// //             <abapsource:template abapsource:name="{source_prog}"/>

// //           </program:abapProgram>"#,
// //                 prog_name = prog_name,
// //                 package_name = package_name,
// //                 source_prog = source_prog_name
// //             ),
// //             prog_name: prog_name.to_string(),
// //         }
// //     }
// // }

// // impl LockHandle for ProgramConfig {
// //     fn get_lock_handle_path(&self) -> String {
// //         format!(
// //             "/sap/bc/adt/programs/programs/{0}?_action=LOCK&accessMode=MODIFY&sap-client=300",
// //             self.prog_name
// //         )
// //     }
// // }

// pub trait LockHandle {
//     fn get_lock_handle_path(&self) -> String;
// }
// // pub struct LockHandle {
// //     body: String,
// //     path: String,
// // }

// // impl LockHandle {
// //     pub fn new(prog_name: &str) -> Self {
// //         LockHandle {
// //             path: format!(
// //                 "/sap/bc/adt/programs/programs/{0}?_action=LOCK&accessMode=MODIFY&sap-client=300",
// //                 prog_name
// //             ),
// //             body: String::new(),
// //         }
// //     }
// // }

// // impl Config for LockHandle {
// //     fn get_body(&self) -> &String {
// //         &self.body
// //     }
// //     fn get_path(&self) -> &String {
// //         &self.path
// //     }
// // }

// impl Config for ProgramConfig {
//     fn get_body(&self) -> &String {
//         &self.body
//     }
//     fn get_path(&self) -> &String {
//         &self.path
//     }
// }
// impl Config for ClassConfig {
//     fn get_body(&self) -> &String {
//         &self.body
//     }
//     fn get_path(&self) -> &String {
//         &self.path
//     }
// }

// // pub struct FreeStyleConfig {
// //     body: String,
// //     path: String,
// // }

// // impl FreeStyleConfig {
// //     pub fn new(sql_exp: String, row_number: Option<u32>) -> Self {
// //         FreeStyleConfig {
// //             body: sql_exp,
// //             path: format!(
// //                 "/sap/bc/adt/datapreview/freestyle?rowNumber={row_number}&sap-client=300",
// //                 row_number = row_number.unwrap_or(5)
// //             ),
// //         }
// //     }
// // }
// // impl Config for FreeStyleConfig {
// //     fn get_body(&self) -> &String {
// //         &self.body
// //     }
// //     fn get_path(&self) -> &String {
// //         &self.path
// //     }
// // }
pub struct ConfigCreateProgram {
    body: String,
    path: String,
    prog_name: String,
}
pub struct ConfigDeleteProgram {
    body: String,
    path: String,
    prog_name: String,
    lock_handle: Option<String>,
    transport_request: String,
}
pub struct ConfigCopyProgram {
    body: String,
    path: String,
    prog_name: String,
}

pub struct ProgramResponse {}

pub struct ProgramError {}

impl ConfigCreateProgram {
    pub fn new(prog_name: &str, package_name: &str, transport_request: &str) -> Self {
        ConfigCreateProgram {
            //ITKK901409
            path: format!(
                "/sap/bc/adt/programs/programs?corrNr={0}&sap-client=300",
                transport_request
            ),
            body: format!(
                r#"<?xml version="1.0" encoding="UTF-8"?>
                        <program:abapProgram xmlns:program="http://www.sap.com/adt/programs/programs" xmlns:adtcore="http://www.sap.com/adt/core" 
                            adtcore:description="Von Programm erstellt" adtcore:language="DE" adtcore:name="{program_name}" adtcore:type="PROG/P" adtcore:masterLanguage="DE" 
                            adtcore:masterSystem="ITK" adtcore:responsible="PFRANK">  
                        
                            <adtcore:packageRef adtcore:name="{package_name}"/>
                    </program:abapProgram>"#,
                program_name = prog_name,
                package_name = package_name
            ),
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
            path: format!(
                "/sap/bc/adt/programs/programs?corrNr={}&sap-client=300",
                transport_request
            ),
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
#[async_trait]
impl Config for ConfigDeleteProgram {
    fn get_body(&self) -> String {
        self.body.clone()
    }
    fn get_path(&self) -> String {
        format!(
            "/sap/bc/adt/programs/programs/{0}?corrNr={1}&lockHandle={2}&sap-client=300",
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
impl Sendable<ProgramResponse, ProgramError> for ConfigDeleteProgram {
    async fn send_with(&mut self, client: &mut SAPClient) -> Result<ProgramResponse, ProgramError> {
        let lock_handle_res = client.send(self).await;

        let xml = lock_handle_res.text().await.unwrap();
        // println!("{:?}", &xml);
        let lock_handle: LockHandleResponse = quick_xml::de::from_str(&xml).unwrap();

        self.lock_handle = Some(lock_handle.values.DATA.LOCK_HANDLE);

        client.delete(self).await;

        Ok(ProgramResponse {})
    }
}
#[async_trait]
impl Sendable<ProgramResponse, ProgramError> for ConfigCreateProgram {
    async fn send_with(&mut self, client: &mut SAPClient) -> Result<ProgramResponse, ProgramError> {
        client.send(self).await;
        Ok(ProgramResponse {})
    }
}
#[async_trait]
impl Sendable<ProgramResponse, ProgramError> for ConfigCopyProgram {
    async fn send_with(&mut self, client: &mut SAPClient) -> Result<ProgramResponse, ProgramError> {
        client.send(self).await;
        Ok(ProgramResponse {})
    }
}
#[async_trait]
impl Config for ConfigCreateProgram {
    fn get_body(&self) -> String {
        self.body.clone()
    }
    fn get_path(&self) -> String {
        self.path.clone()
    }
}
#[async_trait]
impl Config for ConfigCopyProgram {
    fn get_body(&self) -> String {
        self.body.clone()
    }
    fn get_path(&self) -> String {
        self.path.clone()
    }
}
impl LockHandle for ConfigDeleteProgram {
    fn get_lock_handle_path(&self) -> String {
        format!(
            "/sap/bc/adt/programs/programs/{0}?_action=LOCK&accessMode=MODIFY&sap-client=300",
            self.prog_name
        )
    }
}
