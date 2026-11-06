use ratatui::text;

use crate::manifest::Manifest;
use std::io::{self, ErrorKind};
use std::path::Path;

pub fn run(text: &str) -> std::io::Result<()> {
    let path = Path::new("alzh.toml");

    if !path.exists() {
        return Err(io::Error::new(
            ErrorKind::NotFound,
            "Proje klasorunuzde alzh.toml dosyasi bulunmuyor!\nOlusturmak icin 'alzh init' calistirin",
        ));
    }

    let mut manifest = Manifest::load(path)?;

    manifest.project.add_note(text);
    manifest.save(path)?;

    println!("not eklendi.");
    Ok(())
}
