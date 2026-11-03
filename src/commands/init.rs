use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::io::ErrorKind;
use std::process;

// toml tempaltesini alir ve static string olarak iceri aktarir 
static toml_template: &str = include_str!("../../templates/template.toml");


pub fn run() -> std::io::Result<()> {

    // dosyanin adini belirler 
    let path = Path::new("alzh.toml");
    let display = path.display();

    // dosyayi write only olarak ac io::file geri dondurur
    // create yerine create_new kullanildi ki var olan dosyayi ezmesin
    let mut file = match File::create_new(path) {
        Ok(file) => file,
        Err(why) if why.kind() == ErrorKind::AlreadyExists => {
            eprintln!("hata: {} zaten var!", display);
            process::exit(1);
        },
        Err(why) => {
            eprintln!("hata: {} dosyasi olusturulamadi: {}", display, why);
            process::exit(1);
        } 
    };

    // static degiskendeki dosyayi buraya yazar toml_template degisken adi 
    match file.write_all(toml_template.as_bytes()) {
        Err(why) => panic!("dossya yazilamadi"),
        Ok(_) => println!("basariyla dosya yazildi: {}", display),
    }

   Ok(())
} 

/*static kasarliktoml: &str = "
[project]
name="${name}"
version="${version}"
tags=[]
notes=[]
";
*/

