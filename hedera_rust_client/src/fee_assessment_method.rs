#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FeeAssessmentMethod {
    INCLUSIVE,
    EXCLUSIVE,
}

impl From<FeeAssessmentMethod> for bool {
    fn from(f: FeeAssessmentMethod) -> bool {
        match f {
            FeeAssessmentMethod::INCLUSIVE => false,
            FeeAssessmentMethod::EXCLUSIVE => true,
        }
    }
}

impl From<bool> for FeeAssessmentMethod {
    fn from(b: bool) -> FeeAssessmentMethod {
        if b {
            FeeAssessmentMethod::EXCLUSIVE
        } else {
            FeeAssessmentMethod::INCLUSIVE
        }
    }
}
