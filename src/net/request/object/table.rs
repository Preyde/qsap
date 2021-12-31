use async_trait::async_trait;
use reqwest::{Body, Method, StatusCode};

// use super::{strategy::DefaultStrategy, Response, SendWith, Source};
// use super::{Responses, TryFromAsync};
// use crate::config::AdtError;
use crate::{
    data::abap_table::SoapResponse,
    net::request::{Response, Responses},
};

pub struct Table {
    tab_name: String,
    trkorr: String,
    package: String,
}

pub struct TableResponse {
    body: String,
    status: StatusCode,
}

impl Response for TableResponse {
    // type Result = SoapResponse;
    fn get_status(&self) -> StatusCode {
        self.status.clone()
    }
    fn get_text(&self) -> String {
        self.body.clone()
    }
    fn get_value(&self) -> Responses {
        let x: SoapResponse = quick_xml::de::from_str(&self.body).unwrap();
        Responses::Table(x)
    }
}
// #[async_trait]
// impl TryFromAsync<reqwest::Response> for TableResponse {
//     type Error = AdtError;

//     async fn try_from_async(res: reqwest::Response) -> Result<Self, AdtError> {
//         let status = &res.status();
//         if let Ok(text) = res.text().await {
//             Ok(TableResponse {
//                 body: text,
//                 status: status.clone(),
//             })
//         } else {
//             Err(AdtError {
//                 details: String::from("xxx"),
//             })
//         }
//     }
// }

// impl Table {
//     fn new(tab_name: &str, trkorr: &str, package: &str) -> Self {
//         Table {
//             tab_name: tab_name.to_string(),
//             trkorr: trkorr.to_string(),
//             package: package.to_string(),
//         }
//     }
// }

// impl Source<TableResponse, SoapResponse> for Table {
//     fn get_source(&self) -> String {
//         String::new()
//     }
//     fn update_source(&self, source: &str) -> Box<dyn SendWith<SoapResponse>> {}
//     fn source(&self) -> Box<dyn super::Request> {
//         Box::new(DefaultStrategy::new(
//             format!(
//                 r#"<SOAP-ENV:Envelope xmlns:SOAP-ENV="http://schemas.xmlsoap.org/soap/envelope/" xmlns:x="urn:sap-com:document:sap:rfc:functions">
//         <SOAP-ENV:Header></SOAP-ENV:Header>
//         <SOAP-ENV:Body>
//             <x:BDL_DDIF_TABL_GET>
//                 <NAME>{}</NAME>

//                 <DD03P_TAB/>
//             </x:BDL_DDIF_TABL_GET>
//         </SOAP-ENV:Body>
//     </SOAP-ENV:Envelope>"#,
//                 self.tab_name
//             ),
//             String::from("/sap/bc/soap/rfc"),
//             Method::GET,
//         ))
//     }
// }
