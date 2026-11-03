/*
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
*/


