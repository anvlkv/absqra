use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::env;
// extern crate ra;
use ra_parser;
// use ra_parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = Path::new(&args[1]);
    // println!("{:?}", &path);
    let mut file = File::open(path).unwrap();
    
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    // let bts:() = file.bytes();

    let mut prog = ra_parser::parser::parse(&contents);

    println!("{:?}", prog);
}
