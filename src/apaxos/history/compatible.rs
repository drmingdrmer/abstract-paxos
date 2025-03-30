pub trait Compatible<Rhs = Self> {
    fn is_conflicting_with(&self, other: &Rhs) -> bool {
        !self.is_compatible_with(other)
    }

    fn is_compatible_with(&self, other: &Rhs) -> bool;
}
