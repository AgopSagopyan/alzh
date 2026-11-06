pub mod parse;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct Todo {
    #[serde(default)]
    pub items: Vec<String>,
}
