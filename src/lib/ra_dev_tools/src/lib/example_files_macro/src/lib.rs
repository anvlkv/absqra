extern crate proc_macro;
use files_iterator::for_each_ra_example_file;
use proc_macro::{TokenStream, TokenTree};

#[proc_macro_attribute]
pub fn make_example_tests(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut final_stream = TokenStream::new();

    let tokens = item.clone().into_iter().collect::<Vec<TokenTree>>();

    let fn_position = tokens
        .iter()
        .position(|tok| {
            match tok {
                TokenTree::Ident(ident) => {
                    format!("{}", ident) == "fn"
                }
                _ => false,
            }
        })
        .unwrap();

    let fn_argument_start_position = tokens
        .iter()
        .position(|tok| match tok {
            TokenTree::Group(group) => format!("{:?}", group.delimiter()) == "Parenthesis",
            _ => false,
        })
        .unwrap();

    let fn_body_start_position = tokens
        .iter()
        .position(|tok| {
            match tok {
                TokenTree::Group(group) => {
                    format!("{:?}", group.delimiter()) == "Brace"
                }
                _ => false,
            }
        })
        .unwrap();

    for_each_ra_example_file(|contents, file_name| {
        let mut tokens_iter = tokens.clone().into_iter().enumerate();

        while let Some((index, token)) = tokens_iter.next() {
            if index <= fn_position || index > fn_body_start_position{
                final_stream.extend(TokenStream::from(token))
            }
            else if index == fn_position + 1 {
                match token {
                    TokenTree::Ident(id) => {
                        let fn_name = format!("{}_{}", id, file_name).parse::<TokenStream>().unwrap();
                        final_stream.extend(fn_name);
                    },
                    _ => panic!(format!("expected identifier, got {:?}", token))
                }
            }
            else if index == fn_argument_start_position {
                let arguments_list = "()".parse::<TokenStream>().unwrap();
                final_stream.extend(arguments_list);
            }
            else if index == fn_body_start_position {
                match token.clone() {
                    TokenTree::Group(group) => {
                        let original_name_for_contents = {
                            match &tokens[fn_argument_start_position] {
                                TokenTree::Group(group) => {
                                    format!("{}", group.stream().into_iter().collect::<Vec<TokenTree>>()[0])
                                },
                                _ => panic!(format!("expected group, got {:?}", token))
                            }
                        };

                        let original_name_for_file_name = {
                            match &tokens[fn_argument_start_position] {
                                TokenTree::Group(group) => {
                                    format!("{}", group.stream().into_iter().collect::<Vec<TokenTree>>()[4])
                                },
                                _ => panic!(format!("expected group, got {:?}", token))
                            }
                        };

                        let body_stream = format!("{{
                            let {} = \"{}\";
                            let {} = \"{}\";
                            {}
                        }}", 
                        original_name_for_contents, contents.escape_unicode(), 
                        original_name_for_file_name, file_name,
                        group.stream().to_string()).parse::<TokenStream>().unwrap();
                        
                        final_stream.extend(body_stream);

                    },
                    _ => panic!(format!("expected group, got {:?}", token))
                }
            }
        }
    });

    final_stream
}
