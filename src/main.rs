use oshiete::{load_txt, new_pt, tokenize};

fn main() {
    let macbeth = load_txt("data/Macbeth.txt");
    let tokens = tokenize(macbeth.as_str());
    let pt = new_pt(tokens) ;
    println!("{:?}", pt.get("the"));
}
