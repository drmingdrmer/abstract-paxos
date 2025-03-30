use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BinaryOrdering {
    Less,
    Greater,
}

impl From<Ordering> for BinaryOrdering {
    fn from(ordering: Ordering) -> Self {
        match ordering {
            Ordering::Less => BinaryOrdering::Less,
            Ordering::Equal => panic!("Cannot convert Ordering::Equal to BinaryOrdering"),
            Ordering::Greater => BinaryOrdering::Greater,
        }
    }
}
