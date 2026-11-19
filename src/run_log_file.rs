use serde::Serialize;
use std::error::Error;
use std::{fs::OpenOptions, path::Path};

const HEADERS: [&str; 4] = ["Date", "Distance", "Duration", "Pace"];

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Record<'a> {
    pub date: &'a str,
    pub distance: &'a str,
    pub duration: f64,
    pub pace: &'a str,
}

/// Create a new running log containing only the header row.
///
/// Fails if the file already exists, so an existing log is never overwritten.
pub fn create_run_log_file(path: &Path) -> Result<(), String> {
    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|e| format!("Unable to create {}: {e}", path.display()))?;

    let mut log_file = csv::Writer::from_writer(file);

    log_file
        .write_record(HEADERS)
        .map_err(|e| format!("Unable to write headers to {}: {e}", path.display()))?;

    log_file
        .flush()
        .map_err(|e| format!("Unable to write {}: {e}", path.display()))
}

/// Add a new record to the running log.
pub fn add_record(path: &Path, record: Record) -> Result<(), Box<dyn Error>> {
    let file = OpenOptions::new().create(true).append(true).open(path)?;

    let mut writer = csv::WriterBuilder::new()
        .has_headers(false)
        .from_writer(file);

    writer.serialize(Record {
        date: record.date,
        distance: record.distance,
        duration: record.duration,
        pace: record.pace,
    })?;

    writer.flush()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn headers_match_record_fields() {
        let mut writer = csv::Writer::from_writer(vec![]);
        writer
            .serialize(Record {
                date: "2026-11-16",
                distance: "3.16",
                duration: 18.0,
                pace: "5:42",
            })
            .unwrap();

        let csv = String::from_utf8(writer.into_inner().unwrap()).unwrap();
        let header = csv.lines().next().unwrap();

        assert_eq!(header, HEADERS.join(","));
    }
}
