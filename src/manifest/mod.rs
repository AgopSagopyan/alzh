pub mod agent;
pub mod project;
pub mod todo;

use agent::Agent;
use project::Project;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::Path;
use todo::Todo;

#[derive(Serialize, Deserialize, Default)]
pub struct Manifest {
    pub project: Project,
    #[serde(default)]
    pub todo: Todo,
    #[serde(default)]
    pub agent: Agent,
    // tanimadigimiz bolumleri korur
    #[serde(flatten)]
    pub extra: toml::Table,
}

impl Manifest {
    // alzh.toml dosyasini okur, yoksa bos Manifest dondurur
    pub fn load(path: &Path) -> io::Result<Self> {
        if !path.exists() {
            return Ok(Self::default());
        }

        let contents = fs::read_to_string(path)?;
        toml::from_str(&contents).map_err(io::Error::other)
    }

    // Manifest degerini alzh.toml dosyasina yazar
    pub fn save(&self, path: &Path) -> io::Result<()> {
        let contents = toml::to_string_pretty(self).map_err(io::Error::other)?;
        fs::write(path, contents)
    }
}
