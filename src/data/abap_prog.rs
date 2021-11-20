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
    pub fn into_xml(&self) -> String {
        let mut result = String::new();
        for line in self.lines.iter() {
            result.push_str(&format!("<item><LINE>{}</LINE></item>", line));
        }
        result
    }
}
