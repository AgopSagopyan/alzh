use std::fs;
use std::path::{Path, PathBuf};

// bu klasorlerin icine girmeye gerek yok
const SKIP_DIRS: [&str; 3] = ["target", "node_modules", ".git"];

// root altindaki tum alzh.toml dosyalarini bulur
pub fn find_manifests(root: &Path, depth: usize) -> Vec<PathBuf> {
    let mut found = Vec::new();

    let file = root.join("alzh.toml");
    if file.is_file() {
        found.push(file);
    }

    if depth == 0 {
        return found;
    }

    // okunamayan klasorleri sessizce atlar
    let Ok(entries) = fs::read_dir(root) else {
        return found;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        let name = entry.file_name();

        if path.is_dir() && !SKIP_DIRS.iter().any(|skip| name == *skip) {
            found.extend(find_manifests(&path, depth - 1));
        }
    }

    found
}
