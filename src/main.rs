use std::collections::HashMap;
use std::fs;

fn read_todo() -> std::io::Result<HashMap<String, String>> {
    // dosyayi acar
    let contents = fs::read_to_string("")?;

    let mut vars = HashMap::new();

    // dosyayi satir satir okur
    // bos olan ve yorum olan satirlari atlar
    for line in contents.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') { continue; }

        if let Some((key, val)) = line.split_once(':') {
            vars.insert(key.trim().to_string(), val.trim().to_string());
        }
    }

    Ok(vars)
}

use std::fs::File;
use std::io::prelude::*;

mod init;
use init::initialize_file;


//fn main() -> std::io::Result<()> {
//    let mut file = File::create("foo.txt")?;
//    file.write_all(b"Hello, world!")?;
//    Ok(())
//}
//
//
//


fn main() -> std::io::Result<()> {

    initialize_file();
    /*    let vars = read_todo()?;
    println!("t1 = {}", vars["t1"]);
    if let Some(t2) = vars.get("t2") {
        println!("t2 = {t2}");
    }
    //read_todo();
    */
    Ok(())
}


fn old_read_todo() -> std::io::Result<()> {

    let mut file = File::open("foo.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    assert_eq!(contents, "Hello, world!");
    Ok(())
}

