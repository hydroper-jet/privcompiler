use std::rc::Rc;
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Metadata {
    pub name: String,
    pub entries: Vec<Rc<MetadataEntry>>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct MetadataEntry {
    pub key: Option<String>,
    pub value: Rc<MetadataValue>,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum MetadataValue {
    String(String),
    Number(f64),
    Boolean(bool),
    File {
        filename: String,
        data: Vec<u8>,
    },
    List(Vec<Rc<MetadataEntry>>),
}