use std::collections::HashMap;

pub struct AbapProg {
    lines: Vec<String>,
}
impl AbapProg {
    pub fn new(prog_name: &str) -> Self {
        let mut this = AbapProg { lines: vec![] };
        this.lines.push(format!("REPORT {}.", prog_name));

        this
    }
    pub fn add_line(&mut self, line: &str) -> &mut Self {
        self.lines.push(line.to_string());
        self
    }
    pub fn add_function_call(&mut self, name: &str) -> AbapFunctionCall {
        AbapFunctionCall::new(name, self)
    }
    pub fn into_xml(&self) -> String {
        let mut result = String::new();
        for line in self.lines.iter() {
            result.push_str(&format!("<item><LINE>{}</LINE></item>", line));
        }
        result
    }
}

pub struct AbapFunctionCall<'a, 'b> {
    prog: &'a mut AbapProg,
    name: String,
    exporting_params: HashMap<&'b str, &'b str>,
    tables_params: HashMap<String, String>,
}
impl<'a, 'b> AbapFunctionCall<'a, 'b> {
    pub fn new(name: &str, prog: &'a mut AbapProg) -> Self {
        AbapFunctionCall {
            prog,
            name: name.to_string(),
            exporting_params: HashMap::new(),
            tables_params: HashMap::new(),
        }
    }
    pub fn exporting(&mut self, name: &'b str, value: &'b str) -> &mut Self {
        self.exporting_params.insert(name, value);
        self
    }

    pub fn tables(&mut self, name: &str, value: &str) -> &mut Self {
        self.tables_params
            .insert(name.to_string(), value.to_string());
        self
    }

    pub fn dot(&mut self) -> &mut AbapProg {
        self.prog.add_line(&self.to_string());
        self.prog
    }
}
impl<'a, 'b> ToString for AbapFunctionCall<'a, 'b> {
    fn to_string(&self) -> String {
        let mut result = String::new();

        result.push_str(&format!("CALL FUNCTION '{}' ", self.name));

        if self.exporting_params.len() > 0 {
            result.push_str("EXPORTING ");
            for (name, value) in self.exporting_params.clone() {
                result.push_str(&format!("{} = {} ", &name, &value))
            }
        }
        if self.tables_params.len() > 0 {
            result.push_str("TABLES ");
            for (name, value) in self.tables_params.clone() {
                result.push_str(&format!("{} = {} ", &name, &value))
            }
        }

        result.push_str(".");

        result
    }
}
