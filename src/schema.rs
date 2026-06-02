use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Default)]
pub struct ColumnSchema {
    pub dtype: String,

    // presença
    pub not_null: Option<bool>,
    pub unique: Option<bool>,

    // numéricos
    pub gt: Option<f64>,
    pub ge: Option<f64>,
    pub le: Option<f64>,
    pub lt: Option<f64>,
    pub equal: Option<f64>,
    pub between: Option<[f64; 2]>,
    pub is_in: Option<Vec<String>>,
    pub is_positive: Option<bool>,
    pub is_negative: Option<bool>,
    pub is_finite: Option<bool>,

    // strings
    pub contains: Option<String>,
    pub starts_with: Option<String>,
    pub ends_with: Option<String>,
    pub min_length: Option<usize>,
    pub max_length: Option<usize>,
    pub matches_regex: Option<String>,
    pub length_between: Option<[usize; 2]>,
    pub not_empty: Option<bool>,

    // datas
    pub date_format: Option<String>,
    pub after: Option<String>,
    pub before: Option<String>,
    pub between_dates: Option<[String; 2]>,
}

#[derive(Debug, Deserialize)]
pub struct Schema {
    pub columns: HashMap<String, ColumnSchema>,
}

impl Schema {
    pub fn from_toml(content: &str) -> Result<Self, toml::de::Error> {
        toml::from_str(content)
    }
}
