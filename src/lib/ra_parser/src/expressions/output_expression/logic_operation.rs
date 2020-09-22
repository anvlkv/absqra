use super::*;

#[derive(Serialize, Clone, Debug, PartialEq)]
pub enum LogicOperationKind {
    /// &
    AND,
    /// |  
    OR,
    /// ||   
    XOR,
    /// !&  
    NAND,
    /// ! 
    NOT,
    /// !!  
    NOR,
    /// !|  
    XNOR, 
}

#[derive(Serialize, Clone, Debug)]
pub struct LogicOperation {
    pub kind: Option<LogicOperationKind>,
    #[serde(skip)]
    buffer: Buffer<LogicOperation>,
}

impl<'a> Buffered<'a, LogicOperation> for LogicOperation {
    fn new_candidates_from_token(token: &RaToken<'a>) -> Vec<Rc<LogicOperation>> {
        match token.kind {
            TokenKind::Ampersand => vec![Rc::new(Self {
                kind: Some(LogicOperationKind::AND),
                buffer: Vec::new(),
            })],
            TokenKind::Pipe => vec![
                Rc::new(Self {
                    kind: Some(LogicOperationKind::OR),
                    buffer: Vec::new(),
                }),
                Rc::new(Self {
                    kind: Some(LogicOperationKind::XOR),
                    buffer: Vec::new(),
                }),
            ],
            TokenKind::Exclamation => vec![
                Rc::new(Self {
                    kind: Some(LogicOperationKind::NAND),
                    buffer: Vec::new(),
                }),
                Rc::new(Self {
                    kind: Some(LogicOperationKind::NOT),
                    buffer: Vec::new(),
                }),
                Rc::new(Self {
                    kind: Some(LogicOperationKind::NOR),
                    buffer: Vec::new(),
                }),
                Rc::new(Self {
                    kind: Some(LogicOperationKind::XNOR),
                    buffer: Vec::new(),
                }),
            ],
            _ => vec![],
        }
    }
    fn get_buffer(&self) -> Vec<Rc<LogicOperation>> {
        self.buffer.clone()
    }
}

impl<'a> ParsedByToken<'a, LogicOperation> for LogicOperation {
    fn new(token: RaToken<'a>) -> Result<Box<LogicOperation>, Vec<ParserError>> {
        let mut candidates = Self::new_candidates_from_token(&token);
        if candidates.len() == 1 {
            Ok(Box::new(Rc::make_mut(candidates.first_mut().unwrap()).clone()))
        }
        else if candidates.len() > 1 {
            Ok(Box::new(Self{
                kind: None,
                buffer: candidates
            }))
        }
        else {
            Err(vec![
                ParserError::ExpectedAGotB(
                    format!("{:?}", Self::starts_with_tokens()),
                    format!("{:?}", token),
                    token.position.0,
                    Backtrace::new()
                )
            ])
        }
    }
    
    fn append_token(self, token: RaToken<'a>) -> Result<Box<LogicOperation>, Vec<ParserError>> {
        match self.kind {
            Some(_) => Err(vec![
                ParserError::InvalidExpression(token.position.0, Backtrace::new())
            ]),
            None => {
                let kinds = self.allowed_tokens();
                let mut kind: Option<LogicOperationKind> = None;
                let mut errors = Vec::new();

                if kinds.contains(&token.kind) {
                    kind = Some({
                        match token.kind {
                            TokenKind::Pipe => {
                                if self.buffer.iter().find(|op| op.kind.as_ref().unwrap() == &LogicOperationKind::XOR).is_some() {
                                    LogicOperationKind::XOR
                                }
                                else if self.buffer.iter().find(|op| op.kind.as_ref().unwrap() == &LogicOperationKind::XNOR).is_some() {
                                    LogicOperationKind::XNOR
                                }
                                else {
                                    panic!("ops changed");
                                }
                            },
                            TokenKind::Ampersand => LogicOperationKind::NAND,
                            TokenKind::Exclamation => LogicOperationKind::NOR,
                            _ => {
                                errors.push(
                                    ParserError::ExpectedAGotB(
                                        format!("{:?}", kinds),
                                        format!("{:?}", token),
                                        token.position.0,
                                        Backtrace::new()
                                    )
                                );

                                return Err(errors);
                            }
                        }
                    }) 
                }
                else {
                    errors.push(
                        ParserError::ExpectedAGotB(
                            format!("{:?}", kinds),
                            format!("{:?}", token),
                            token.position.0,
                            Backtrace::new()
                        )
                    )
                }

                

                if errors.len() != 0 {
                    Err(errors)
                }
                else {
                    Ok(Box::new(Self {
                        kind: kind,
                        buffer: vec![]
                    }))
                }
            }
        }
    }
    fn allowed_tokens(&self) -> Vec<TokenKind<'a>> {
        match self.kind {
            Some(_) => vec![],
            None => {
                let mut kinds = Vec::new();
                if self.buffer.iter().find(|op| op.kind.as_ref().unwrap() == &LogicOperationKind::OR).is_some() {
                    kinds.push(TokenKind::Pipe);
                }

                if self.buffer.iter().find(|op| op.kind.as_ref().unwrap() == &LogicOperationKind::NOT).is_some() {
                    kinds.extend(vec![
                        TokenKind::Ampersand,
                        TokenKind::Exclamation,
                        TokenKind::Pipe
                    ]);
                }

                kinds
            }
        }
    }
    fn starts_with_tokens() -> Vec<TokenKind<'a>> {
        vec![
            TokenKind::Ampersand,
            TokenKind::Pipe,
            TokenKind::Exclamation,
        ]
    }
}
