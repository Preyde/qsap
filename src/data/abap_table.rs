use cli_table::{print_stdout, Cell, Color, Style, Table};
use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize, Serializer};
#[derive(Debug)]
pub struct ABAPTable {
    headers: Vec<String>,
    data: Vec<Vec<String>>,
    table_data: Option<TableData>,
}

impl Clone for ABAPTable {
    fn clone(&self) -> Self {
        ABAPTable {
            data: self.data.clone(),
            headers: self.headers.clone(),
            table_data: None,
        }
    }
}
// impl Copy for ABAPTable {}

impl ABAPTable {
    pub fn new(table_data: TableData) -> ABAPTable {
        ABAPTable {
            headers: vec![],
            data: vec![],
            table_data: Some(table_data),
        }
    }
    pub fn build(&mut self) {
        if self.headers.is_empty() {
            self.extract_headers();
        }
        if self.data.is_empty() {
            self.extract_data();
        }
    }
    pub fn get_headers(&self) -> Vec<String> {
        self.headers.to_owned()
    }

    fn extract_headers(&mut self) {
        self.headers = self
            .table_data
            .as_ref()
            .unwrap()
            .columns
            .iter()
            .map(|column: &Columns| String::from(&column.metadata.name))
            // .map(|t: &String| t.cell().bold(true))
            .collect();
    }

    fn extract_data(&mut self) {
        let len = 0..self.table_data.as_ref().unwrap().columns[0]
            .data_set
            .data
            .len();

        let mut i: usize = 0;
        let mut data: Vec<Vec<String>> = vec![];

        loop {
            let mut data_vec: Vec<String> = vec![];

            for col in self.table_data.as_ref().unwrap().columns.iter() {
                let data = &col.data_set.data[i];

                data_vec.push(Self::option_to_string(data));
            }

            if i == len.end - 1 {
                break;
            } else {
                i = i + 1;
                data.push(data_vec);
            }
        }
        self.data = data;
    }

    fn option_to_string(option: &Option<String>) -> String {
        match option {
            Some(val) => val.to_string(),
            None => String::from(""),
        }
    }

    pub fn get_data(&self) -> Vec<Vec<String>> {
        self.data.to_owned()
    }

    pub fn display(self) {
        print_stdout(
            self.data
                .table()
                .title(self.headers.iter().map(|t: &String| t.cell().bold(true))),
        );
    }
}

#[derive(Debug, Deserialize)]
pub struct DataSet {
    data: Vec<Option<String>>,
}
#[derive(Debug, Deserialize)]
pub struct Columns {
    metadata: Metadata,
    #[serde(rename = "dataSet")]
    data_set: DataSet,
}

#[derive(Debug, Deserialize)]
pub struct Metadata {
    #[serde(rename = "dataPreview:name")]
    name: String,
    #[serde(rename = "dataPreview:type")]
    r#type: String,
    #[serde(rename = "dataPreview:description")]
    description: String,
    #[serde(rename = "dataPreview:keyAttribute")]
    keyAttribute: bool,
    #[serde(rename = "dataPreview:colType")]
    colType: String,
    #[serde(rename = "dataPreview:isKeyFigure")]
    isKeyFigure: bool,
}
#[derive(Debug, Deserialize)]
pub struct TableData {
    totalRows: u32,
    isHanaAnalyticalView: bool,
    executedQueryString: String,
    queryExecutionTime: String,
    columns: Vec<Columns>,
}
#[derive(Debug, Deserialize)]
pub struct XML {
    tableData: TableData,
}
#[derive(Debug, Deserialize)]

// pub struct Document {}
pub struct XmlTest {
    #[serde(rename = "SOAP-ENV:Body")]
    res: SoapResponse,
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename = "SOAP-ENV:Envelope")]
// #[serde(rename = "SOAP-ENV:Envelope")]
#[derive(PartialEq, Eq)]
pub struct SoapResponse {
    // header: String,
    // #[serde(rename = "SOAP-ENV:Body")]
    // #[serde(rename = "SOAP-ENV:Envelope")]
    // #[serde(rename = "RFCDEMO:BDL_DDIF_TABL_GET.Response")]
    #[serde(rename = "Body")]
    pub body: SoapBody,
}
#[derive(Debug, Deserialize, Serialize)]
// #[serde(rename = "SOAP-ENV:Body")]
#[derive(PartialEq, Eq)]
pub struct SoapBody {
    #[serde(rename = "BDL_DDIF_TABL_GET.Response")]
    pub response: BdlDdifTablGetResponse,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
// #[serde(rename = "x:BDL_DDIF_TABL_GET.Response")]
pub struct BdlDdifTablGetResponse {
    #[serde(rename = "DD02V_WA")]
    pub dd02v: Dd02v,
    #[serde(rename = "DD09L_WA")]
    pub dd09l: Dd09l,
    #[serde(rename = "DD03P_TAB")]
    pub fields: DD03P_TAB,
}

// struct Dd02v{
//     tab_name: String,
//     dd_language: String,
//     tab_class: String,
//     sql_tab: String,
//     dat_min: String,
//     dat_max: String,
//     dat_avg: String,
//     cli_dep: String,
//     buffered: String,

// }
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename = "DD02V_WA")]
pub struct Dd02v {
    #[serde(rename = "$unflatten=TABNAME")]
    TABNAME: String,
    #[serde(rename = "$unflatten=TABCLASS")]
    TABCLASS: String,
    #[serde(rename = "$unflatten=AS4USER")]
    AS4USER: String,
    #[serde(rename = "$unflatten=AS4DATE")]
    AS4DATE: String,
    #[serde(rename = "$unflatten=AS4TIME")]
    AS4TIME: String,
    #[serde(rename = "$unflatten=CONTFLAG")]
    CONTFLAG: String,
    #[serde(rename = "$unflatten=PROZPUFF")]
    PROZPUFF: String,
    #[serde(rename = "$unflatten=EXCLASS")]
    EXCLASS: String,
}

// impl Serialize for Dd02v {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer,
//     {
//         let mut s = serializer.serialize_struct("DD02V_WA", 8)?;

//         s.serialize_field("TABNAME", &self.TABNAME);
//         s.end()
//     }
// }

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename = "DD09L_WA")]
pub struct Dd09l {
    #[serde(rename = "$unflatten=TABNAME")]
    TABNAME: String,
    #[serde(rename = "$unflatten=AS4LOCAL")]
    AS4LOCAL: String,
    #[serde(rename = "$unflatten=AS4VERS")]
    AS4VERS: String,
    #[serde(rename = "$unflatten=TABKAT")]
    TABKAT: String,
    #[serde(rename = "$unflatten=TABART")]
    TABART: String,
    #[serde(rename = "$unflatten=SCHFELDANZ")]
    SCHFELDANZ: String,
    #[serde(rename = "$unflatten=AS4USER")]
    AS4USER: String,
    #[serde(rename = "$unflatten=AS4DATE")]
    AS4DATE: String,
    #[serde(rename = "$unflatten=AS4TIME")]
    AS4TIME: String,
    #[serde(rename = "$unflatten=BUFALLOW")]
    BUFALLOW: String,
    #[serde(rename = "$unflatten=ROWORCOLST")]
    ROWORCOLST: String,
}
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Dd03pStruc {
    #[serde(rename = "$unflatten=TABNAME")]
    TABNAME: String,
    #[serde(rename = "$unflatten=FIELDNAME")]
    FIELDNAME: String,
    #[serde(rename = "$unflatten=DDLANGUAGE")]
    DDLANGUAGE: String,
    #[serde(rename = "$unflatten=POSITION")]
    POSITION: String,
    #[serde(rename = "$unflatten=KEYFLAG")]
    KEYFLAG: String,
    #[serde(rename = "$unflatten=MANDATORY")]
    MANDATORY: String,
    #[serde(rename = "$unflatten=ROLLNAME")]
    ROLLNAME: String,
    #[serde(rename = "$unflatten=CHECKTABLE")]
    CHECKTABLE: String,
    #[serde(rename = "$unflatten=ADMINFIELD")]
    ADMINFIELD: String,
    #[serde(rename = "$unflatten=INTTYPE")]
    INTTYPE: String,
    #[serde(rename = "$unflatten=INTLEN")]
    INTLEN: String,
    #[serde(rename = "$unflatten=REFTABLE")]
    REFTABLE: String,
    #[serde(rename = "$unflatten=PRECFIELD")]
    PRECFIELD: String,
    #[serde(rename = "$unflatten=REFFIELD")]
    REFFIELD: String,
    #[serde(rename = "$unflatten=CONROUT")]
    CONROUT: String,
    #[serde(rename = "$unflatten=NOTNULL")]
    NOTNULL: String,
    #[serde(rename = "$unflatten=DOMNAME")]
    DOMNAME: String,
    #[serde(rename = "$unflatten=ROUTPUTLEN")]
    ROUTPUTLEN: String,
    #[serde(rename = "$unflatten=MEMORYID")]
    MEMORYID: String,
    #[serde(rename = "$unflatten=LOGFLAG")]
    LOGFLAG: String,
    #[serde(rename = "$unflatten=HEADLEN")]
    HEADLEN: String,
    #[serde(rename = "$unflatten=SCRLEN1")]
    SCRLEN1: String,
    #[serde(rename = "$unflatten=SCRLEN2")]
    SCRLEN2: String,
    #[serde(rename = "$unflatten=SCRLEN3")]
    SCRLEN3: String,
    #[serde(rename = "$unflatten=DTELGLOBAL")]
    DTELGLOBAL: String,
    #[serde(rename = "$unflatten=DTELMASTER")]
    DTELMASTER: String,
    #[serde(rename = "$unflatten=RESERVEDTE")]
    RESERVEDTE: String,
    #[serde(rename = "$unflatten=DATATYPE")]
    DATATYPE: String,
    #[serde(rename = "$unflatten=LENG")]
    LENG: String,
    #[serde(rename = "$unflatten=OUTPUTLEN")]
    OUTPUTLEN: String,
    #[serde(rename = "$unflatten=DECIMALS")]
    DECIMALS: String,
    #[serde(rename = "$unflatten=LOWERCASE")]
    LOWERCASE: String,
    #[serde(rename = "$unflatten=SIGNFLAG")]
    SIGNFLAG: String,
    #[serde(rename = "$unflatten=LANGFLAG")]
    LANGFLAG: String,
    #[serde(rename = "$unflatten=VALEXI")]
    VALEXI: String,
    #[serde(rename = "$unflatten=ENTITYTAB")]
    ENTITYTAB: String,
    #[serde(rename = "$unflatten=CONVEXIT")]
    CONVEXIT: String,
    #[serde(rename = "$unflatten=MASK")]
    MASK: String,
    #[serde(rename = "$unflatten=MASKLEN")]
    MASKLEN: String,
    #[serde(rename = "$unflatten=ACTFLAG")]
    ACTFLAG: String,
    #[serde(rename = "$unflatten=DDTEXT")]
    DDTEXT: String,
    #[serde(rename = "$unflatten=REPTEXT")]
    REPTEXT: String,
    #[serde(rename = "$unflatten=SCRTEXT_S")]
    SCRTEXT_S: String,
    #[serde(rename = "$unflatten=SCRTEXT_M")]
    SCRTEXT_M: String,
    #[serde(rename = "$unflatten=SCRTEXT_L")]
    SCRTEXT_L: String,
    #[serde(rename = "$unflatten=DOMMASTER")]
    DOMMASTER: String,
    #[serde(rename = "$unflatten=RESERVEDOM")]
    RESERVEDOM: String,
    #[serde(rename = "$unflatten=DOMGLOBAL")]
    DOMGLOBAL: String,
    #[serde(rename = "$unflatten=DOMNAME3L")]
    DOMNAME3L: String,
    #[serde(rename = "$unflatten=SHLPORIGIN")]
    SHLPORIGIN: String,
    #[serde(rename = "$unflatten=SHLPNAME")]
    SHLPNAME: String,
    #[serde(rename = "$unflatten=SHLPFIELD")]
    SHLPFIELD: String,
    #[serde(rename = "$unflatten=TABLETYPE")]
    TABLETYPE: String,
    #[serde(rename = "$unflatten=DEPTH")]
    DEPTH: String,
    #[serde(rename = "$unflatten=COMPTYPE")]
    COMPTYPE: String,
    #[serde(rename = "$unflatten=DEFFDNAME")]
    DEFFDNAME: String,
    #[serde(rename = "$unflatten=GROUPNAME")]
    GROUPNAME: String,
    #[serde(rename = "$unflatten=REFTYPE")]
    REFTYPE: String,
    #[serde(rename = "$unflatten=PROXYTYPE")]
    PROXYTYPE: String,
    #[serde(rename = "$unflatten=LANGUFLAG")]
    LANGUFLAG: String,
    #[serde(rename = "$unflatten=EXCLASS")]
    EXCLASS: String,
    #[serde(rename = "$unflatten=LTRFLDDIS")]
    LTRFLDDIS: String,
    #[serde(rename = "$unflatten=BIDICTRLC")]
    BIDICTRLC: String,
    #[serde(rename = "$unflatten=DBPOSITION")]
    DBPOSITION: String,
    #[serde(rename = "$unflatten=ANONYMOUS")]
    ANONYMOUS: String,
    #[serde(rename = "$unflatten=OUTPUTSTYLE")]
    OUTPUTSTYLE: String,
    #[serde(rename = "$unflatten=NOHISTORY")]
    NOHISTORY: String,
    #[serde(rename = "$unflatten=AMPMFORMAT")]
    AMPMFORMAT: String,
    #[serde(rename = "$unflatten=STREAMORLOC")]
    STREAMORLOC: String,
    #[serde(rename = "$unflatten=STRORLOCPOS")]
    STRORLOCPOS: String,
}
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct DD03P_TAB {
    item: Vec<Dd03pStruc>,
}
