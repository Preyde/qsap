use async_trait::async_trait;
use reqwest::{Method, StatusCode};

use crate::{
    data::abap_table::{AbapTable, Dd02v, Dd09l, SoapResponse, TableData, DD03P_TAB},
    net::{
        behavior::{CopyToSys, Details},
        request::{
            strategy::{CopyTabToSysStrategy, DefaultStrategy},
            DefaultResponse, Response, Responses, TryFromAsync,
        },
        AdtError, SendWith,
    },
};

pub struct Table {
    tab_name: String,
    transport_req: Option<String>,
    package: Option<String>,
}

pub struct DetailResponse {
    body: String,
    status: StatusCode,
}
impl Response for DetailResponse {
    // type Result = SoapResponse;
    fn get_status(&self) -> StatusCode {
        self.status.clone()
    }
    fn get_text(&self) -> String {
        self.body.clone()
    }
    fn get_value(&self) -> Responses {
        let x: SoapResponse = quick_xml::de::from_str(&self.body).unwrap();

        Responses::Detail(x)
    }
}
#[async_trait]
impl TryFromAsync<reqwest::Response> for DetailResponse {
    async fn try_from_async(res: reqwest::Response) -> Result<Box<dyn Response>, AdtError> {
        let status = res.status();
        if let Ok(text) = res.text().await {
            Ok(Box::new(DetailResponse { body: text, status }))
        } else {
            Err(AdtError {
                details: String::from("xxx"),
            })
        }
    }
}

pub struct TableResponse {
    body: String,
    status: StatusCode,
}

impl Response for TableResponse {
    fn get_status(&self) -> StatusCode {
        self.status.clone()
    }
    fn get_text(&self) -> String {
        self.body.clone()
    }
    fn get_value(&self) -> Responses {
        let x: TableData = quick_xml::de::from_str(&self.body).unwrap();
        let mut tab = AbapTable::new(x);
        tab.build();
        Responses::Table(tab)
    }
}

#[async_trait]
impl TryFromAsync<reqwest::Response> for TableResponse {
    // type Error = AdtError;

    async fn try_from_async(res: reqwest::Response) -> Result<Box<dyn Response>, AdtError> {
        let status = res.status();
        if let Ok(text) = res.text().await {
            // println!("{}", text);
            Ok(Box::new(TableResponse { body: text, status }))
        } else {
            Err(AdtError {
                details: String::from("xxx"),
            })
        }
    }
}

impl Table {
    pub fn new(tab_name: &str, transport_req: Option<&str>, package: Option<&str>) -> Self {
        Table {
            tab_name: tab_name.to_string(),
            transport_req: match transport_req {
                Some(v) => Some(v.to_string()),
                None => None,
            },
            package: match package {
                Some(v) => Some(v.to_string()),
                None => None,
            },
        }
    }
    pub fn freestyle(sql: &str, rows: Option<u32>) -> Box<dyn SendWith> {
        let x: DefaultStrategy<TableResponse> = DefaultStrategy::new(
            sql.to_string(),
            format!(
                "/sap/bc/adt/datapreview/freestyle?rowNumber={}",
                rows.unwrap_or(5)
            ),
            Method::POST,
        );
        Box::new(x)
    }
}

impl Details for Table {
    fn details(&self) -> Box<dyn SendWith> {
        let body = format!(
            r#"<SOAP-ENV:Envelope xmlns:SOAP-ENV="http://schemas.xmlsoap.org/soap/envelope/" xmlns:x="urn:sap-com:document:sap:rfc:functions">
                        <SOAP-ENV:Header></SOAP-ENV:Header>
                        <SOAP-ENV:Body>
                            <x:BDL_DDIF_TABL_GET>
                                <NAME>{}</NAME>
            
                                <DD03P_TAB/>
                            </x:BDL_DDIF_TABL_GET>
                        </SOAP-ENV:Body>
                    </SOAP-ENV:Envelope>"#,
            self.tab_name
        );

        let x: DefaultStrategy<DetailResponse> =
            DefaultStrategy::new(body, String::from("/sap/bc/soap/rfc"), Method::POST);
        Box::new(x)
    }
    fn update_details(&self, dd02v: &Dd02v, dd09l: &Dd09l, dd03p: &DD03P_TAB) -> Box<dyn SendWith> {
        let body = format!(
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
            self.transport_req.as_ref().unwrap_or(&String::from("$TMP")),
            self.package.as_ref().unwrap_or(&String::new()),
            self.tab_name,
            quick_xml::se::to_string(dd02v).unwrap(),
            quick_xml::se::to_string(dd09l).unwrap(),
            quick_xml::se::to_string(dd03p).unwrap()
        );

        let x: DefaultStrategy<DefaultResponse> =
            DefaultStrategy::new(body, String::from("/sap/bc/soap/rfc"), Method::POST);
        Box::new(x)
    }
}

impl CopyToSys for Table {
    fn copy_to_sys<'a>(&'a self, dest: &crate::net::Destination) -> Box<dyn SendWith + 'a> {
        let x: CopyTabToSysStrategy<Table, DefaultResponse> = CopyTabToSysStrategy::new(self, dest);
        Box::new(x)
    }
}
