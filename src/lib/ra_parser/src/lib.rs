use ra_lexer::tokenize;

pub fn parse(&input: String) {
    let mut stream = tokenize(input);

    
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
