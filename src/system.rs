use crate::apaxos::history::mono_history::MonoHistory;
use crate::Types;

/// A [`System`] is type container that contains derived types from [`Types`].
/// while [`Types`] contains only primitive types.
pub trait System {
    type Types: Types;

    type MonoHistory: MonoHistory<Self::Types>;
}
