use std::fs;

fn main() {
    let json = fs::read_to_string("data.json").expect("No data.json");
    println!("{}", json)
}
