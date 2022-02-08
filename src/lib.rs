use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

pub fn load_txt(path: &str) -> String {
    let mut f = File::open(path).expect(&*("file not found: ".to_owned() + path));
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    contents
}

pub fn tokenize(text: &str) -> Vec<String> {
    text.split_whitespace()
        .map(|s| s.to_string()).collect()
}

pub fn new_pt(tokens: &Vec<String>) -> HashMap<String, Vec<usize>> {
    let mut pt: HashMap<String, Vec<usize>> = HashMap::new();
    for (i, token) in tokens.iter().enumerate() {
        let term = token.to_lowercase();
        match pt.get_mut(&term) {
            Some(positions) => {
                positions.push(i);
            }
            None => {
                pt.insert(term, vec![i]);
            }
        }
    }
    pt
}