pub struct CsvWriter {
    out: Vec<String>,
}

impl CsvWriter {
    pub fn new() -> Self {
        Self { out: Vec::new() }
    }

    pub fn write_record(&mut self, fields: Vec<&str>) {
        let line = fields
            .iter()
            .map(|f| {
                if f.contains(',') || f.contains('"') || f.contains('\n') {
                    format!("\"{}\"", f.replace('"', "\"\""))
                } else {
                    f.to_string()
                }
            })
            .collect::<Vec<_>>()
            .join(",");
        self.out.push(line);
    }

    pub fn print(&self) {
        for line in &self.out {
            println!("{}", line);
        }
    }
}
