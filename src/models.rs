use serde::{Serialize,Deserialize};

#[derive(Debug,Serialize,Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Mode{
    CASH,
    UPI,
    CHEQUE
}

#[derive(Debug,Serialize,Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectionModel{
    pub grp_name: String,
    pub member_name: String,
    pub amount: f32,
    pub mode: Mode,
    pub cheque_no: Option<String>,
    pub bank: Option<String>,
}