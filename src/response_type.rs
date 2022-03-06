#[derive(FromPrimitive, ToPrimitive, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ResponseType {
    /// Response returns answer
    AnswerOnly = 0,
    /// (NOT YET SUPPORTED) Response returns both answer and state proof
    AnswerStateProof = 1,
    /// Response returns the cost of answer
    CostAnswer = 2,
    /// (NOT YET SUPPORTED) Response returns the total cost of answer and state proof
    CostAnswerStateProof = 3,
}
