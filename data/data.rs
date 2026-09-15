use serde::Deserialize;
use std::collections;

// {"features":[{"feature_idx":0,"name":"text","type":{"dtype":"string","_type":"Value"}}],"rows":[],"num_rows_total":97230848,"num_rows_per_page":100,"partial"

#[derive(Deserialize, Debug)]
struct FeatureType {
    dtype: String,
    _type: String,
}

#[derive(Deserialize, Debug)]
struct Feature {
    feature_idx: u32,
    name: String,
    #[serde(rename = "type")]
    data_type: FeatureType,
}

#[derive(Deserialize, Debug)]
struct DataSetRows {
    features: Feature,
    rows: Vec<serde_json::Value>,
    num_rows_total: u64,
    num_rows_per_page: u32,
    partial: bool,
}

struct DataSet {
    base_url: String,
}

impl DataSet {
    pub fn new(url: String) -> Self {
        Self { base_url: url };
    }
    pub fn fetch(&self, offset: u32, len: u32) {
        let query_params = [
            ("dataset", "karpathy/fineweb-edu-100b-shuffle"),
            ("config", "default"),
            ("split", "train"),
            ("offset", &offset.to_string()),
            ("length", &length.to_string()),
        ];
        let client = reqwest::blocking::client::new();
        let response: DataSetRows = client
            .get(base_url)
            .query(&query_params)
            .send()?
            .error_for_status()?
            .json::<DataSetRows>()?;
        Ok(response)
    }
}
