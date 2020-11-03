use super::*;

#[derive(Serialize, Debug, Clone)]
pub enum ComparisonOperator {
    EqCompare,
    GtCompare,
    LsCompare,
    GtEqCompare,
    LsEqCompare,
    NEqCompare,
}