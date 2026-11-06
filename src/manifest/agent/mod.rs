use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct Agent {
    #[serde(default)]
    pub rules: Vec<String>,
}
