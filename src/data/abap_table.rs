use cli_table::{print_stdout, Cell, Color, Style, Table};
use serde::Deserialize;
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
