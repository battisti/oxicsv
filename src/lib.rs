use csv::StringRecord;
use std::error::Error;
use std::fs::File;
use std::path::{Path, PathBuf};

fn get_path_to_csv() -> PathBuf {
    let mut local_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    local_path.push("data");
    local_path.push("opcua_signals.csv");
    local_path
}

fn read_csv_from_path(path: &Path) -> Result<Vec<StringRecord>, Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(path)?;

    Ok(reader.records().filter_map(Result::ok).collect())
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

}
