use super::Sendable;
use super::{AdtError, Config, Responses, SAPClient, SendableConfig};
use async_trait::async_trait;

#[derive(Debug)]
pub struct ClassConfig {
    body: String,
    path: String,
}

pub struct ClassResponse {}

pub struct ClassError {}
// impl AdtError for ClassError {}

impl ClassConfig {
    pub fn new(class_name: &str, package_name: &str, transport_request: &str) -> Self {
        ClassConfig {
            //ITKK901409
            path: format!(
                "/sap/bc/adt/oo/classes?corrNr={0}&sap-client=300",
                transport_request
            ),
            body: format!(
                r#"<?xml version="1.0" encoding="UTF-8"?><class:abapClass xmlns:class="http://www.sap.com/adt/oo/classes" xmlns:adtcore="http://www.sap.com/adt/core" adtcore:description="xxx" adtcore:language="DE" adtcore:name="{class_name}" adtcore:type="CLAS/OC" adtcore:masterLanguage="DE" adtcore:masterSystem="ITK" adtcore:responsible="PFRANK" class:final="true" class:visibility="public">
    
                <adtcore:packageRef adtcore:name="{package_name}"/>
                  
                <class:include adtcore:name="CLAS/OC" adtcore:type="CLAS/OC" class:includeType="testclasses"/>
                  
                <class:superClassRef/>
                
              </class:abapClass>"#,
                class_name = class_name,
                package_name = package_name
            ),
        }
    }
}

impl Config for ClassConfig {
    fn get_body(&self) -> String {
        self.body.clone()
    }
    fn get_path(&self) -> String {
        self.path.clone()
    }
}
#[async_trait]
impl Sendable for ClassConfig {
    async fn send_with(&mut self, client: &mut SAPClient) -> Result<(), AdtError> {
        let res = client.send(self).await;
        Ok(())
        // Ok(res.text().await.unwrap())
    }
    fn get_response(&self) -> Option<Responses> {
        Some(Responses::Class(String::from("")))
    }
}
impl SendableConfig for ClassConfig {}
