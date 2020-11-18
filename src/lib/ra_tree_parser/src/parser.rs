use super::*;
use ra_lexer::errors::LexerError;
use indextree::{Arena, Node};
// use super::tree::RaTree;

pub fn parse<'a>(
    mut tokens_stream: impl Iterator<Item = Result<RaToken, LexerError>>
) -> Result<Arena<RaBlock>, (Vec<LexerError>, Option<Arena<RaBlock>>)> {

    let mut tree = Arena::<RaBlock>::new();

    let mut errors: Vec<LexerError> = Vec::new();
    let mut awaiting_paired_tokens = Vec::new();

    while let Some(token_result) = tokens_stream.next() {
        match token_result {
            Ok(token) => {
                // let last_added_block = {
                //     match tree.iter().last() {
                //         Some(l) => Some(l.),
                //         None => None
                //     }
                // };
                
                let token_id = tree.new_node(RaBlock::Token(token.clone()));
                
                let mut block_id = tree.new_node(RaBlock::Block);

                if let Some(pair) = token_pair(&token.kind) {
                    awaiting_paired_tokens.push(pair);
                    let group_id = tree.new_node(RaBlock::Group);
                    block_id.append(group_id, &mut tree);
                    block_id = group_id;
                }



                // match last_added_block {
                //     Some(last_block) => {
                //         // last_block. ;
                // //         match last_block.last_child() {
                // //             Some(child_id) => {

                // //             },
                // //             None => {
                // //                 
                // //             }
                // //         }
                // //         // last_block

                // //         // last_block;
                //     },
                //     None => {
                //         block_id.append(token_id, &mut tree);
                //     }
                // }
                /*
                 a + b + c
                    node - Block
                        a - Token
                        b - Token
                        c - Token

                (a + b) + c
                    node - Block
                        (a + b) - Group
                            a - Token
                            + - Token
                            b - Token
                        
                        + - Token
                        c - Token

                something
                    a + b

                node - Block
                    something - Token
                        node - Block
                            a - Token
                            + - Token
                            b - Token

                 */
            },
            Err(e) => {
                errors.push(e);
            }
        }
    }
    if errors.len() > 0 {
        Err((errors, Some(tree)))
    }
    else {
        Ok(tree)
    }
}

fn token_pair(kind: &TokenKind) -> Option<TokenKind> {
    match kind {
        TokenKind::OpenCurlyBrace => Some(TokenKind::CloseCurlyBrace),
        TokenKind::OpenSquareBrace => Some(TokenKind::CloseSquareBrace),
        TokenKind::OpenParentheses => Some(TokenKind::CloseParentheses),
        _ => None,
    }
}

fn is_child(token: &RaToken, node: &Node<RaBlock>, arena: &Arena<RaBlock>) -> bool {
    match node.get() {
        RaBlock::Block
        | RaBlock::Group => {
            match node.last_child() {
                Some(child_id) => {
                    is_child(token, arena.get(child_id).unwrap(), arena)
                },
                None => false
            }
        },
        RaBlock::Token(last_token) => {
            token.level > last_token.level
        }
    }
}

fn is_sibling(token: &RaToken, node: &Node<RaBlock>, arena: &Arena<RaBlock>) -> bool {
    match node.get() {
        RaBlock::Block
        | RaBlock::Group => {
            match node.last_child() {
                Some(child_id) => {
                    is_sibling(token, arena.get(child_id).unwrap(), arena)
                },
                None => false
            }
        },
        RaBlock::Token(last_token) => {
            token.level == last_token.level && !is_current(token, node, arena)
        }
    }
}


fn is_current(token: &RaToken, node: &Node<RaBlock>, arena: &Arena<RaBlock>) -> bool {
    match node.get() {
        RaBlock::Block
        | RaBlock::Group => {
            match node.last_child() {
                Some(child_id) => {
                    is_current(token, arena.get(child_id).unwrap(), arena)
                },
                None => false
            }
        },
        RaBlock::Token(last_token) => {
            token.level == last_token.level && (token.position.0).0 == (last_token.position.0).0
        }
    }
}

