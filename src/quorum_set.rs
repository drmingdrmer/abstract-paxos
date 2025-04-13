use core::fmt;

use crate::Types;

pub trait QuorumSet<T: Types>
where Self: fmt::Display
{
    fn get_read_quorum(&self) -> Vec<T::AcceptorId>;
    fn get_write_quorum(&self) -> Vec<T::AcceptorId>;

    fn is_read_quorum(&self, acceptor_ids: impl IntoIterator<Item = T::AcceptorId>) -> bool;
    fn is_write_quorum(&self, acceptor_ids: impl IntoIterator<Item = T::AcceptorId>) -> bool;
}
