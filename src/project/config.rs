use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::Path;

#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    pub project: Project,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Project {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub todos: Vec<String>,
    // tanimadigimiz alanlari korur, yoksa kaydederken silinirler
    #[serde(flatten)]
    pub extra: toml::Table,
}

impl Config {
    pub fn load(path: &Path) -> io::Result<Self> {
        if !path.exists() {
            return Ok(Self::default());
        }

        let contents = fs::read_to_string(path)?;
        toml::from_str(&contents).map_err(io::Error::other)
    }
     // Config degerini alzh.toml dosyasina yazar
    pub fn save(&self, path: &Path) -> io::Result<()> {
        let contents = toml::to_string_pretty(self).map_err(io::Error::other)?;
        fs::write(path, contents)
    }
}
