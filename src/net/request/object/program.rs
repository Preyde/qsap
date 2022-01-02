use crate::net::{
    behavior::{CopyTo, CopyToSys, Create, Delete, Source},
    request::{
        strategy::{CopyToSysStrategy, DefaultStrategy, LockStrategy},
        DefaultResponse, LockObject, SendWith,
    },
    Destination,
};

use reqwest::Method;

#[derive(Debug)]
pub struct Program {
    body: String,
    path: String,
    prog_name: String,
    package_name: Option<String>,
    transport_request: Option<String>,
    // lock_handle: Option<String>,
    source: Option<String>,
}

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
            // lock_handle: None,
            source: None,
        }
    }
}
impl CopyToSys for Program {
    // type Response = DefaultResponse;
    fn copy_to_sys<'a>(&'a self, dest: &Destination) -> Box<dyn SendWith + 'a> {
        let x: Box<CopyToSysStrategy<Program, DefaultResponse>> = Box::new(CopyToSysStrategy::new(
            // LockObject::new(&"/sap/bc/adt/programs/programs/"),
            self,
            dest.clone(),
        ));
        x
    }
}

impl Source for Program {
    // type Response = DefaultResponse;
    fn source(&self) -> Box<dyn SendWith> {
        let x: Box<DefaultStrategy<DefaultResponse>> = Box::new(DefaultStrategy::new(
            String::new(),
            format!(
                "/sap/bc/adt/programs/programs/{}/source/main",
                self.prog_name
            ),
            Method::GET,
        ));
        x
    }
    fn update_source(&self, source: &str) -> Box<dyn SendWith> {
        let x: Box<LockStrategy<DefaultResponse>> = Box::new(LockStrategy::new(
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
        ));
        x
    }
    fn get_source(&self) -> String {
        String::new()
    }
}

impl Create for Program {
    fn create(&self) -> Box<dyn SendWith> {
        let x: Box<DefaultStrategy<DefaultResponse>> = Box::new(DefaultStrategy::new(
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
        ));
        x
    }
}

impl CopyTo for Program {
    // type Response = DefaultResponse;
    fn copy_to(&self, target_name: &str) -> Box<dyn SendWith> {
        let x: Box<DefaultStrategy<DefaultResponse>> = Box::new(DefaultStrategy::new(
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
        ));
        x
    }
}

impl Delete for Program {
    // type Response = DefaultResponse;
    fn delete(&mut self) -> Box<dyn SendWith> {
        let x: Box<LockStrategy<DefaultResponse>> = Box::new(LockStrategy::new(
            String::new(),
            Method::DELETE,
            LockObject::new(&format!("/sap/bc/adt/programs/programs/{}", self.prog_name)),
        ));
        x
    }
}

// #[derive(Debug)]
// pub struct ConfigExecuteProgram {
//     body: String,
//     path: String,
// }

// impl ConfigExecuteProgram {
//     fn new(prog: &AbapProg) -> Self {
//         ConfigExecuteProgram {
//             path: String::from("/sap/bc/soap/rfc"),
//             body: format!(
//                 r#"<SOAP-ENV:Envelope xmlns:SOAP-ENV="http://schemas.xmlsoap.org/soap/envelope/" xmlns:x="urn:sap-com:document:sap:rfc:functions">
//     <SOAP-ENV:Header></SOAP-ENV:Header>
//     <SOAP-ENV:Body>
//         <x:RFC_ABAP_INSTALL_AND_RUN>
//             <PROGRAM>{}</PROGRAM>
//         </x:RFC_ABAP_INSTALL_AND_RUN>
//     </SOAP-ENV:Body>
// </SOAP-ENV:Envelope>"#,
//                 prog.into_xml()
//             ),
//         }
//     }
// }
// impl SendableConfig for ConfigExecuteProgram {}
// #[async_trait]
// impl Sendable for ConfigExecuteProgram {
//     async fn send_with(&mut self, client: &mut SAPClient) -> Result<(), AdtError> {
//         // let res = client.send(self).await;
//         // println!("{}", res.status());
//         // println!("{}", res.text().await.unwrap());
//         Ok(())
//     }
//     fn get_response(&self) -> Option<Responses> {
//         Some(Responses::Program(String::from("Program executed")))
//     }
// }
// impl Config for ConfigExecuteProgram {
//     fn get_body(&self) -> String {
//         self.body.clone()
//     }
//     fn get_path(&self) -> String {
//         self.path.clone()
//     }
// }
