use crate::apaxos::history::binary_order::BinaryOrdering;

pub trait BinaryCompare {
    fn compare(&self, other: &Self) -> BinaryOrdering;

    fn is_greater_than(&self, other: &Self) -> bool {
        self.compare(other) == BinaryOrdering::Greater
    }

    fn is_less_than(&self, other: &Self) -> bool {
        self.compare(other) == BinaryOrdering::Less
    }
}
