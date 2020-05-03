


use super::expression::Expression;

#[derive(Clone, Debug, PartialEq)]
pub enum BlockKind {
    Program,
    Input,
    Output,
    RuleDeclaration,
    RuleInvocation,
    Reference,
    ContextDeclaration,
    Content,
    Union,
    Undetermined,
}

impl Default for BlockKind {
    fn default() -> Self {
        BlockKind::Undetermined
    }
}

#[derive(Clone, Debug, Default)]
pub struct Block<'a> {
    pub kind: BlockKind,
    pub expression: Expression<'a>,
    pub (crate) children: Vec<Block<'a>>,
    pub level: u16
}