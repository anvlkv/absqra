extern crate scrawl;
use scrawl::{editor};
use ra_tree_parser::parser::parse;
use ra_lexer::tokenize;
use ra_parser::ast::RaAST;
use std::panic;
use std::convert::TryFrom;

fn main() {
    let mut content = String::from("// enter ra code after this line \n\n\n // keep this line to quit \n ra_quit");

    let mut output = String::new();

    let mut quit = false;

    while !quit {

        let input = editor::new()
                        .contents(&content)
                        .open()
                        .unwrap();
        

        let trimmed_input = &input[..input.find("==================\n\n\n").unwrap_or(input.len())];
        
        
        if trimmed_input.trim().len() > 0 {
            let tree = parse(tokenize(trimmed_input));
            let result = panic::catch_unwind(|| RaAST::try_from(tree.unwrap()));
    
            output = match result {
                Ok(r) => format!("OUTPUT: {:#?}", r),
                Err(e) => format!("ERROR: {:#?}", e)
            };
        }

        content = trimmed_input.to_owned();

        content.push_str("==================\n\n\n");
        content.push_str("/* OUTPUT *");
        content.push_str(&output);
        content.push_str("* END OF OUTPUT */");


        quit = trimmed_input.find("ra_quit").is_some();
    }

    println!("{}", output);
}
