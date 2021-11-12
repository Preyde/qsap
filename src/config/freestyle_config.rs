use super::{AdtError, AdtResponse, Config, Responses, SAPClient};
use crate::config::Sendable;
use crate::data::abap_table::{ABAPTable, TableData};
use async_trait::async_trait;
use reqwest::Response;
pub struct FreeStyleConfig {
    body: String,
    path: String,
}

pub struct FreeStyleResponse {
    table: ABAPTable,
}

impl AdtResponse for FreeStyleResponse {
    fn get_data(self) -> Responses {
        Responses::FreeStyle(self.table)
    }
}
// impl AdtResponse<ABAPTable> for FreeStyleResponse {
//     fn get_data(&self) -> ABAPTable {
//         self.table
//     }
// }
pub struct FreeStyleError {}
impl AdtError for FreeStyleError {}
// impl FreeStyleResponse {
//     fn get_table(&self) -> ABAPTable {
//         self.table
//     }
// }

impl FreeStyleConfig {
    pub fn new(sql_exp: String, row_number: Option<u32>) -> Self {
        FreeStyleConfig {
            body: sql_exp,
            path: format!(
                "/sap/bc/adt/datapreview/freestyle?rowNumber={row_number}&sap-client=300",
                row_number = row_number.unwrap_or(5)
            ),
        }
    }
}
#[async_trait]
impl Config for FreeStyleConfig {
    fn get_body(&self) -> String {
        self.body.clone()
    }
    fn get_path(&self) -> String {
        self.path.clone()
    }
}

#[async_trait]
impl Sendable<FreeStyleResponse, FreeStyleError> for FreeStyleConfig {
    async fn send_with(
        &mut self,
        client: &mut SAPClient,
    ) -> Result<FreeStyleResponse, FreeStyleError> {
        let res = client.send(self).await;

        let xml = res.text().await.unwrap();
        let table_data: TableData = quick_xml::de::from_str(&xml).unwrap();

        let mut abap_table = ABAPTable::new(table_data);

        abap_table.build();

        let response = crate::config::FreeStyleResponse { table: abap_table };
        Ok(response)
    }
}
// impl Into<Config> for FreeStyleConfig {
//     fn into(self) -> Config {
//         Config {}
//     }
// }
// #[async_trait]
// impl SendWith for FreeStyleConfig {

// }
