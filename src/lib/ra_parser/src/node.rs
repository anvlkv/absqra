use super::*;

#[derive(Serialize, Debug, Clone)]
pub enum RaASTNode {
    Root,
    Annotation(AnnotationExpression),
    Content(ContentExpression),
    Context(ContextExpression),
    Input(InputExpression),
    Output(OutputExpression),
    Reference(ReferenceExpression),
}

impl RaASTNode {
    pub (crate) fn parse(tokens: &[RaToken]) -> Result<Self, Vec<ParserError>> {
        match tokens.first() {
            Some(token) => {
                match tokens {
                    t if AnnotationExpression::accepts_tokens(t) => {
                        Ok(Self::Annotation(AnnotationExpression::parse(tokens)?))
                    },
                    t if ContentExpression::accepts_tokens(t) => {
                        Ok(Self::Content(ContentExpression::parse(tokens)?))
                    },
                    t if ContextExpression::accepts_tokens(t) => {
                        Ok(Self::Context(ContextExpression::parse(tokens)?))
                    },
                    t if InputExpression::accepts_tokens(t) => {
                        Ok(Self::Input(InputExpression::parse(tokens)?))
                    },
                    t if OutputExpression::accepts_tokens(t) => {
                        Ok(Self::Output(OutputExpression::parse(tokens)?))
                    },
                    t if ReferenceExpression::accepts_tokens(t) => {
                        Ok(Self::Reference(ReferenceExpression::parse(tokens)?))
                    },
                    _ => Err(vec![ParserError::UnexpectedToken(
                        format!("{:?}", token.kind),
                        token.position.0,
                        Backtrace::new()
                    )])
                }
            },
            None => panic!("Called parse with empty tokens")
        }
    }

    pub fn level(&self) -> u16 {
        match self {
            Self::Root => 0,
            Self::Annotation(expression) => expression.level(),
            Self::Content(expression) => expression.level(),
            Self::Context(expression) => expression.level(),
            Self::Input(expression) => expression.level(),
            Self::Output(expression) => expression.level(),
            Self::Reference(expression) => expression.level(),
        }
    }

    pub fn position(&self) -> (Position, Position) {
        match self {
            Self::Root => (Position::default(), Position::default()),
            Self::Annotation(expression) => expression.position(),
            Self::Content(expression) => expression.position(),
            Self::Context(expression) => expression.position(),
            Self::Input(expression) => expression.position(),
            Self::Output(expression) => expression.position(),
            Self::Reference(expression) => expression.position(),
        }
    }
}