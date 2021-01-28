extern crate scrawl;
use scrawl::editor;
use ra_lexer::tokenize;
use ra_parser::parse;

fn main() {
    let mut content = String::from("// enter ra code");

    let mut output = String::new();

    let mut quit = false;

    while !quit {

        let input = editor::new()
                        .contents(&content)
                        .open()
                        .unwrap();
        

        let trimmed_input = &input[..input.find("==================\n\n\n").unwrap_or(input.len())];
        
        
        if trimmed_input.trim().len() > 0 {
            let result = parse(tokenize(trimmed_input));
    
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
