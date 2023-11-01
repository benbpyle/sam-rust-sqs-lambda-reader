use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MainModel {
    id: String,
    name: String,
    description: String,
    custom_note: String
}
