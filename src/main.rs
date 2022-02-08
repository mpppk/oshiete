use std::cmp;
use oshiete::{load_txt, new_pt, tokenize};

fn main() {
    let macbeth = load_txt("data/Macbeth.txt");
    let tokens = tokenize(macbeth.as_str());
    let pt = new_pt(&tokens) ;
    let empty_posting_list: &Vec<usize> = &vec![];
    // if let Some(pl) = pt.get("king") {
    //     posting_list = pl
    // }
    let posting_list = pt.get("king").unwrap_or(empty_posting_list);
    println!("{:?}", posting_list.iter().map(|p| {
        let min = cmp::max(p-3, 0);
        let max = cmp::min(p+3, tokens.len());
        tokens[min..max].iter()
            .map(|t| t.as_str())
            .collect::<Vec<&str>>()
            .join(" ")
    }).collect::<Vec<String>>())
}
