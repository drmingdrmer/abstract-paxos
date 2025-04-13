use binary_compare::BinaryCompare;
use compatible::Compatible;
use futures::StreamExt;
use mono_history::MonoHistory;

use crate::Types;
use crate::acceptor::Client;
use crate::quorum_set::QuorumSet;
use crate::system::System;

pub mod binary_compare;
pub mod binary_order;
pub mod compatible;
pub mod mono_history;

pub trait History<T: Types>
where Self: Compatible + 'static
{
    type Mono: MonoHistory<T>;

    fn mono_histories(&self) -> Vec<Self::Mono>;
}
