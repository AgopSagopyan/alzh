// todo.txt iceridigini satir satir listeye cevirir
pub fn parse(contents: &str) -> Vec<String> {
    contents
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(str::to_string)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bos_satirlari_atlar() {
        let todos = parse("sut al\n\n kod yaz  \n");
        assert_eq!(todos, vec!["sut al", "kod yaz"]);
    }
}
