// handles file reading & data parsing
use anyhow::{Context, Result};
pub struct Reader {
    rdr: csv::Reader<std::fs::File>,
    headers: Vec<String>,
}
impl Reader {
    pub fn headers(&self) -> &[String] { &self.headers }
    pub fn index_of(&self, name: &str) -> Result<usize> {
        self.headers.iter().position(|h| h == name)
            .with_context(|| format!("column not found: {name}"))
    }
    pub fn next_record(&mut self) -> Result<Option<Vec<String>>> {
        let mut row = csv::StringRecord::new();
        if self.rdr.read_record(&mut row)? {
            Ok(Some(row.iter().map(|s| s.to_string()).collect()))
        } else { Ok(None) }
    }
}
pub fn open(path: &str) -> Result<Reader> {
    let file = std::fs::File::open(path).with_context(|| format!("open {path}"))?;
    let mut rdr = csv::Reader::from_reader(file);
    let headers = rdr.headers()?.iter().map(|s| s.to_string()).collect();
    Ok(Reader { rdr, headers })
}
