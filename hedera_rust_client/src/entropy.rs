use crate::proto::services::{self};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Entropy {
    PrngBytes(Vec<u8>),
    PrngNumber(i32),
}

impl From<services::transaction_record::Entropy> for Entropy {
    fn from(services: services::transaction_record::Entropy) -> Entropy {
        match services {
            services::transaction_record::Entropy::PrngBytes(bytes) => Entropy::PrngBytes(bytes),
            services::transaction_record::Entropy::PrngNumber(num) => Entropy::PrngNumber(num),
        }
    }
}
