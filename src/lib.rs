use std::error::Error;
use std::path::{Path, PathBuf};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Record {
    metric_id: String,
    metric_unit: String,
    asset_id: String,
    opc_ns: u32,
    opc_id: String,
    notes: String,
}

fn get_path_to_csv() -> PathBuf {
    let mut local_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    local_path.push("data");
    local_path.push("opcua_signals.csv");
    local_path
}

fn read_csv_from_path(path: &Path) -> Result<Vec<Record>, Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(path)?;

    Ok(reader.deserialize().filter_map(Result::ok).collect())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_locate_csv_file() {
        use std::fs::{metadata, Metadata};

        let path = get_path_to_csv();
        let is_file = metadata(&path).as_ref().map(Metadata::is_file).unwrap();

        assert!(is_file);
    }

    #[test]
    fn should_read_all_records() {
        let path = get_path_to_csv();
        let records = read_csv_from_path(&path).unwrap();

        assert_eq!(records.len(), 550);
    }

    #[test]
    fn should_deserialize_records() {
        let path = get_path_to_csv();
        let metric_id = read_csv_from_path(&path)
            .unwrap()
            .get(0)
            .map(|r| r.metric_id.clone());

        assert_eq!(metric_id, Some(format!("053A0LBD07CP901XQ01")))
    }
}
