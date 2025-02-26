use std::fs::File;
use std::io::prelude::*;

struct Config {
    name: String,
    id: u32
}
impl Config {
    fn from_file(path: &str) -> Config {
        let mut file = File::open(path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let mut lines = contents.lines();
        let name = lines.next().unwrap().to_string();
        let id: u32 = lines.next().expect("Missing ID").parse().unwrap();

        Config { name, id }
    }
}
fn reading_from_file() {
    let config = Config::from_file("my_files/config.txt");
    println!("name: {}", config.name);
    println!("ID: {}", config.id);
}

fn main() {
    reading_from_file();
}