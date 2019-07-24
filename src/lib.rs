use lazy_static::lazy_static;
use serde::Deserialize;
use serde_json::json;

lazy_static! {
    pub static ref RECORDS: Vec<Record> = get_records();
}

static CSV_DATA: &'static [u8; 43205] = include_bytes!("../data/opcua_signals.csv");

#[derive(Debug, Deserialize)]
pub struct Record {
    metric_id: String,
    metric_unit: String,
    asset_id: String,
    opc_ns: u32,
    opc_id: String,
    notes: String,
}

impl Record {
    pub fn to_json(&self) -> serde_json::Value {
        let id = format!("ns={};id={}", self.opc_ns, self.opc_id);

        json!({
            "id": self.metric_id,
            "unit": self.metric_unit,
            "signalId": id,
            "comment": self.notes,
        })
    }
}

fn get_records() -> Vec<Record> {
    let rdr: &[u8] = CSV_DATA.as_ref();
    let mut reader = csv::Reader::from_reader(rdr);

    reader.deserialize().filter_map(Result::ok).collect()
}

pub fn get_json_records() -> serde_json::Value {
    let records: Vec<_> = RECORDS.iter().map(Record::to_json).collect();

    json!(records)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_read_all_records() {
        assert_eq!(get_records().len(), 550);
    }

    #[test]
    fn should_deserialize_records() {
        let metric_id = get_records().get(0).map(|r| r.metric_id.clone());

        assert_eq!(metric_id, Some(format!("053A0LBD07CP901XQ01")))
    }

    #[test]
    fn should_serialize_json() {
        let actual = get_records().get(0).map(Record::to_json).unwrap();

        let expected = json!({
             "id": "053A0LBD07CP901XQ01",
            "unit": "bar",
            "signalId": "ns=4;id=053 A0 LBD07 CP901%#_#%%#_#%XQ01",
            "comment": "external - bar",
        });

        assert_eq!(expected, actual);
    }
}
