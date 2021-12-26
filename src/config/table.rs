use async_trait::async_trait;
use reqwest::{Body, Method, StatusCode};

use super::TryFromAsync;
use super::{strategy::DefaultStrategy, Response, SendWith, Source};
use crate::config::AdtError;
use crate::data::abap_table::SoapResponse;

pub struct Table {
    tab_name: String,
    trkorr: String,
    package: String,
}

pub struct TableResponse {
    body: String,
    status: StatusCode,
}

// impl Response<SoapResponse> for TableResponse {
//     fn get_status(&self) -> StatusCode {
//         self.status.clone()
//     }
//     fn get_text(&self) -> String {
//         self.body.clone()
//     }
//     fn get_value(&self) -> SoapResponse {
//         quick_xml::de::from_str(&self.body).unwrap()
//     }
// }
// #[async_trait]
// impl TryFromAsync<reqwest::Response> for TableResponse {
//     type Error = AdtError;

//     async fn try_from_async(res: reqwest::Response) -> Result<Self, AdtError> {
//         if let Ok(text) = res.text().await {
//             Ok(TableResponse {
//                 body: text,
//                 status: res.status(),
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
