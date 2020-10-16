use super::*;

#[derive(Serialize, Debug, Clone)]
pub enum RaASTNode {
    Root,
    Output,
    Input,
    Annotation,
    Context,
    Content
}