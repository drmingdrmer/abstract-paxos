use std::collections::BTreeSet;
use std::collections::HashSet;

use binary_compare::BinaryCompare;
use compatible::Compatible;
use mono_history::MonoHistory;

use crate::System;
use crate::Types;

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

pub trait Distributed<S: System> {
    type Acceptor;

    fn get_read_quorum(&self) -> Vec<S::Types::AcceptorId>;

    fn get_write_quorum(&self) -> Vec<S::Types::AcceptorId>;

    fn get_acceptor(&self, id: S::Types::AcceptorId) -> Self::Acceptor;

    fn read(&self) -> HashSet<S::MonoHistory> {
        let targets = self.get_read_quorum();

        self.read_from_nodes(&targets)
    }

    fn read_from_nodes(&self, nodes: &[S::Types::AcceptorId]) -> HashSet<S::MonoHistory> {
        let mut monos = Vec::new();
        for node in nodes {
            monos.extend(node.read());
        }
        monos
    }

    fn write_to_nodes(&mut self, targets: &[S::Types::AcceptorId], h: S::MonoHistory) {
        for node in targets {
            node.write(h.clone());
        }
    }
}

pub trait Universe<S: System>
where Self: Distributed<S>
{
    fn read(&self) -> HashSet<S::MonoHistory> {
        let monos = Distributed::read(self);
        S::MonoHistory::observe(monos)
    }
}
