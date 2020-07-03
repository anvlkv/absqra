use ra_parser::parser::parse;
use std::panic;
use std::io::{stdin, stdout, Write};

fn main() {
    loop {
        print!("ra_ ");
        stdout().flush();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        

        if input.trim().len() > 0 {
            let result = panic::catch_unwind(|| parse(&input));
    
            match result {
                Ok(r) => println!("{:?}", r),
                Err(e) => println!("{:?}", e)
            }
        }

    }
}
