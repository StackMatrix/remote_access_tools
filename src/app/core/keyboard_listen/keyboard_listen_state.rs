use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KeyboardRecordStruct {
    pub id: usize,
    pub value: String,
    pub datetime: String,
}