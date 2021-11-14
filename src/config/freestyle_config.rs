use super::{AdtError, Config, Responses, SAPClient, SendableConfig};
use crate::config::Sendable;
use crate::data::abap_table::{ABAPTable, TableData};
use async_trait::async_trait;
pub struct FreeStyleConfig {
    body: String,
    path: String,
    table: Option<ABAPTable>,
}

pub struct FreeStyleResponse {
    table: Option<ABAPTable>,
}

pub struct FreeStyleError {}

impl FreeStyleConfig {
    pub fn new(sql_exp: String, row_number: Option<u32>) -> Self {
        FreeStyleConfig {
            body: sql_exp,
            path: format!(
                "/sap/bc/adt/datapreview/freestyle?rowNumber={row_number}&sap-client=300",
                row_number = row_number.unwrap_or(5)
            ),
            table: None,
        }
    }
}

impl Config for FreeStyleConfig {
    fn get_body(&self) -> String {
        self.body.clone()
    }
    fn get_path(&self) -> String {
        self.path.clone()
    }
}
#[async_trait]
impl Sendable for FreeStyleConfig {
    async fn send_with(&mut self, client: &mut SAPClient) -> Result<(), AdtError> {
        let res = client.send(self).await;

        let xml = res.text().await.unwrap();

        let table_data: TableData = match quick_xml::de::from_str(&xml) {
            Ok(data) => data,
            Err(_e) => return Err(AdtError::new("Table does not exist")),
        };

        let mut abap_table = ABAPTable::new(table_data);

        abap_table.build();
        self.table = Some(abap_table);
        Ok(())
        // Ok(abap_table)
    }
    fn get_response(&self) -> Option<Responses> {
        match self.table.clone() {
            Some(t) => Some(Responses::FreeStyle(t)),
            None => None,
        }
    }
}

impl SendableConfig for FreeStyleConfig {}
