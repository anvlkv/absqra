use super::*;


#[derive(Serialize, Debug, Clone)]
pub enum LogicOperator {
    AND,  // &
    OR,   // |
    NOT,  // !
    NAND, // !&
    NOR,  // !!
    XOR,  // ||
    XNOR, // !|
}