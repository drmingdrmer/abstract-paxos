use std::collections::BTreeSet;
use std::collections::HashSet;

use binary_compare::BinaryCompare;
use compatible::Compatible;
use mono_history::MonoHistory;

use crate::quorum_set::QuorumSet;
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

    type QuorumSet: QuorumSet<S::Types>;

    fn get_quorum_set(&self) -> &Self::QuorumSet;

    fn get_acceptor(&self, id: &S::Types::AcceptorId) -> Self::Acceptor;

    fn read(&self) -> HashSet<S::MonoHistory> {
        let targets = self.get_quorum_set().get_read_quorum();
        self.read_from_nodes(&targets)
    }

    fn read_from_nodes(&self, nodes: &[S::Types::AcceptorId]) -> HashSet<S::MonoHistory> {
        let mut monos = Vec::new();
        for node in nodes {
            let acceptor = self.get_acceptor(node);
            let fu = acceptor.read();
            monos.extend(acceptor.read());
        }
        monos
    }

    fn write_to_nodes(&mut self, targets: &[S::Types::AcceptorId], h: S::MonoHistory) {
        for node in targets {
            let acceptor = self.get_acceptor(node);
            acceptor.write(h.clone());
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
