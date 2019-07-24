use std::path::PathBuf;

fn get_path_to_csv() -> PathBuf {
    let mut local_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    local_path.push("data");
    local_path.push("opcua_signals.csv");
    local_path
}

#[cfg(test)]
mod test {

    use super::get_path_to_csv;
    use std::fs::{metadata, Metadata};
    use std::io::Error;

    #[test]
    fn get_path_to_csv_should_locate_file() {
        let path = get_path_to_csv();
        let is_file = metadata(&path).as_ref().map(Metadata::is_file).unwrap();

        assert!(is_file);
    }

}
