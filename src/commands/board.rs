use serde::de::Error;

use crate::manifest;
use crate::manifest::Manifest;
use crate::manifest::project::Project;
use crate::scan;
use crate::tui;
use std::env;
use std::io;
use std::mem::min_align_of;
use std::path::{Path, PathBuf};

// maksimum klasor derinligi
const MAX_DEPTH: usize = 3;

pub fn run(path: Option<&Path>, cli: bool) -> io::Result<()> {
    if let Some(path) = path {
        if !cli {
            let manifest_path = path.join("alzh.toml");

            if !manifest_path.exists() {
                return Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    "Bu klasorde alzh.toml dosyasi bulunmuyor!\nOlusturmak icin 'alzh init' calistirin",
                ));
            }

            return tui::app::run(&manifest_path);
        }
    }

    let home = env::var("HOME").map_err(io::Error::other)?;
    let root = PathBuf::from(home).join("Projects");

    let mut manifests = scan::find_manifests(&root, MAX_DEPTH);

    if manifests.is_empty() {
        println!("{} altinda alzh.toml bulunamadi.", root.display());
        println!("Bir proje klasorunde 'alzh init' calistirin");
        return Ok(());
    }

    for path in manifests {
        match Manifest::load(&path) {
            Ok(manifest) => print!("{:<20} {}", manifest.project.name, path.display()),
            Err(why) => eprintln!("uyari: {} okunamadi: {why}", path.display()),
        }
    }

    Ok(())
}
