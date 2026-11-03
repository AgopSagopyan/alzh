use crate::project::{Config, todo};
use std::fs;
use std::io::{self, ErrorKind};
use std::path::Path;

pub fn run() -> std::io::Result<()> {
    let todo_path = Path::new("todo.txt");
    let config_path = Path::new("alzh.toml");

    let contents = fs::read_to_string(todo_path).map_err(|why| {
        if why.kind() == ErrorKind::NotFound {
            io::Error::new(
                why.kind(),
                "Proje klasorunuzde todo.txt dosyasi bulunmuyor!\nOlusturup her satira bir gorev yazin sonra 'alzh import' calistirin.",
            )
        } else {
            why
        }
    })?;

    let mut config = Config::load(config_path)?;
    config.project.todos = todo::parse(&contents);
    config.save(config_path)?;

    println!("{} todo aktarildi", config.project.todos.len());
    Ok(())
}
