use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct Project {
    #[serde(default)]
    pub name: String,
    // tanimadigimiz alanlari korur, yoksa kaydederken silinirler
    #[serde(flatten)]
    pub extra: toml::Table,
    #[serde(default)]
    pub notes: Vec<String>,
}

impl Project {
    // notlarin sonuna yeni not ekler
    pub fn add_note(&mut self, text: &str) {
        self.notes.push(text.to_string());
    }
}
