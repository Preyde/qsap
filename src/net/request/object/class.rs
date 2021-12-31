use reqwest::Method;

use crate::net::{
    behavior::Create,
    request::{strategy::DefaultStrategy, DefaultResponse, SendWith},
};

// use super::{strategy::DefaultStrategy, Create, DefaultResponse, SendWith};

pub struct Class {
    name: String,
    package_name: Option<String>,
    transport_request: Option<String>,
}

impl Class {
    pub fn new(name: &str, transport_request: Option<&str>, package_name: Option<&str>) -> Self {
        Class {
            name: name.to_string(),
            package_name: match package_name {
                Some(v) => Some(v.to_string()),
                None => None,
            },
            transport_request: match transport_request {
                Some(v) => Some(v.to_string()),
                None => None,
            },
        }
    }
}

impl Create for Class {
    fn create(&self) -> Box<dyn SendWith> {
        let strategy: DefaultStrategy<DefaultResponse> = DefaultStrategy::new(
            format!(
                r#"<?xml version="1.0" encoding="UTF-8"?><class:abapClass xmlns:class="http://www.sap.com/adt/oo/classes" xmlns:adtcore="http://www.sap.com/adt/core" adtcore:description="xxx" adtcore:language="DE" adtcore:name="{class_name}" adtcore:type="CLAS/OC" adtcore:masterLanguage="DE" adtcore:masterSystem="ITK" adtcore:responsible="PFRANK" class:final="true" class:visibility="public">
    
                <adtcore:packageRef adtcore:name="{package_name}"/>
                  
                <class:include adtcore:name="CLAS/OC" adtcore:type="CLAS/OC" class:includeType="testclasses"/>
                  
                <class:superClassRef/>
                
              </class:abapClass>"#,
                class_name = self.name,
                package_name = self.package_name.as_ref().unwrap_or(&String::from("$TMP"))
            ),
            match &self.transport_request {
                Some(t) => format!("/sap/bc/adt/oo/classes?corrNr={0}", t),
                None => String::from("/sap/bc/adt/oo/classes"),
            },
            Method::POST,
        );
        Box::new(strategy)
    }
}
