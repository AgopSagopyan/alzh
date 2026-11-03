use std::fs;
use std::io::{self, ErrorKind};

pub fn run(name: &str) -> std::io::Result<()> {
    let contents = fs::read_to_string("alzh.toml").map_err(|why| {
        if why.kind() == ErrorKind::NotFound {
            io::Error::new(why.kind(), "Proje klasorunuzde alzh.toml dosyasi bulunmuyor! \n Olusturmak icin 'alzh init' calistirin.",
            )
        } else {
            why
        }
    })?;
    println!("{name} okundu : {contents}");
    Ok(())
}
