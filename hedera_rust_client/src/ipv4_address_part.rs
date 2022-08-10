use std::fmt::{self};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IPv4AddressPart {
    pub left: u8,
    pub right: u8,
}

impl fmt::Display for IPv4AddressPart {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}", self.left, self.right)
    }
}
