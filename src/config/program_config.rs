pub trait Config {
    fn get_path(&self) -> &String;
    fn get_body(&self) -> &String;
}

pub struct ProgramConfig {
    body: String,
    path: String,
}

impl ProgramConfig {
    pub fn new(prog_name: &String) -> Self {
        ProgramConfig {
            path: String::from("/sap/bc/adt/programs/programs?corrNr=ITKK901409&sap-client=300"),
            body: format!(
                r#"<?xml version="1.0" encoding="UTF-8"?><program:abapProgram xmlns:program="http://www.sap.com/adt/programs/programs" xmlns:adtcore="http://www.sap.com/adt/core" adtcore:description="Von Programm erstellt" adtcore:language="DE" adtcore:name="{program_name}" adtcore:type="PROG/P" adtcore:masterLanguage="DE" adtcore:masterSystem="ITK" adtcore:responsible="PFRANK">  <adtcore:packageRef adtcore:name="Z_PFRANK"/></program:abapProgram>"#,
                program_name = prog_name
            ),
        }
    }
}

impl Config for ProgramConfig {
    fn get_body(&self) -> &String {
        &self.body
    }
    fn get_path(&self) -> &String {
        &self.path
    }
}

pub struct FreeStyleConfig {
    body: String,
    path: String,
}

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
impl Config for FreeStyleConfig {
    fn get_body(&self) -> &String {
        &self.body
    }
    fn get_path(&self) -> &String {
        &self.path
    }
}
