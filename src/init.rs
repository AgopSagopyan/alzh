static kasarliktoml: &str = "

project-name = '{}'
tags = '{}'
    
";

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;


pub fn initialize_file() {
    
    let path = Path::new("kasarlik.toml");
    let display = path.display();

    // dosyayi write only olarak ac io::file geri dondurur
    let mut file = match File::create(&path) {
        Err(why) => panic!("{} dosyasi olusturulamadi: {}", display, why),
        Ok(file) => file,
    };

    // static degiskendeki dosyayi buraya yazar 
    match file.write_all(kasarliktoml.as_bytes()) {
        Err(why) => panic!("dossya yazilamadi"),
        Ok(_) => println!("basariyla dosya yazildi: {}", display),
    }
}
