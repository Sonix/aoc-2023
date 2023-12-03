pub fn read_lines_from_file(filename: &str) -> Vec<String> {
    match std::fs::read_to_string(filename) {
        Ok(contents)=> contents.lines().map(|x| {x.to_owned()}).collect(),
        _ => panic!("File {} not found!", filename)
    }
}