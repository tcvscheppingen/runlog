use serde::Serialize;
use std::{fs::OpenOptions, path::Path};

const HEADERS: [&str; 3] = ["Date", "Distance", "Pace"];

// Not written yet by the CLI; the header test keeps HEADERS in sync with it.
#[allow(dead_code)]
#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
struct Record<'a> {
    date: &'a str,
    distance: f64,
    pace: &'a str,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn headers_match_record_fields() {
        let mut writer = csv::Writer::from_writer(vec![]);
        writer
            .serialize(Record {
                date: "2026-11-16",
                distance: 3160.0,
                pace: "5:42",
            })
            .unwrap();

        let csv = String::from_utf8(writer.into_inner().unwrap()).unwrap();
        let header = csv.lines().next().unwrap();

        assert_eq!(header, HEADERS.join(","));
    }
}
