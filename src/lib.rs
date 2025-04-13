#![feature(associated_type_defaults)]
#![feature(map_try_insert)]
extern crate core;

pub mod aliases;
pub mod apaxos;
pub mod commonly_used;
pub mod implementations;
mod quorum_set;
pub(crate) mod sealed;
mod system;

pub mod acceptor;
mod distributed;
pub mod errors;
mod universe;

use std::collections::BTreeMap;
use std::fmt::Debug;

use apaxos::proposal::Proposal;
use apaxos::ptime::Time;
use quorum_set::QuorumSet;

use crate::apaxos::acceptor::Acceptor;
use crate::apaxos::history::mono_history::MonoHistory;
use crate::sealed::Sealed;

pub trait AcceptorId: Debug + Clone + Copy + Ord + 'static {}

pub trait Value: Debug + Clone + 'static {}

/// Defines types that are used in the Abstract-Paxos algorithm.
pub trait Types: Debug + Clone + Sized + 'static {
    /// Acceptor ID
    type AcceptorId: AcceptorId = u64;

    /// Pseudo time used in a distributed consensus.
    ///
    /// Every distributed consensus algorithm has its own definition of time.
    /// - In Paxos, it is ballot number, which is `(round, proposer_id)`.
    /// - In Raft, it is `(term, Option<voted_for>)`.
    /// - In 2PC, it is mainly a vector of related data entry name.
    // TODO: explain 2pc time.
    type Time: Time;

    /// The value to propose and to commit
    type Value: Value;

    /// A part of the [`Proposal`] data that is stored on an [`Acceptor`].
    type Part: Value;

    /// Quorum set defines quorums for read and write.
    ///
    /// Read-quorum is used by phase-1, write-quorum is used by phase-2.
    /// In most cases, read-quorum and write-quorum are the same.
    ///
    /// A quorum set defines the cluster structure.
    // TODO: explain cluster structure
    type QuorumSet: QuorumSet<Self>;

    /// The network transport for sending and receiving messages.
    type Transport: Transport<Self>;
}

pub trait Transport<T: Types> {
    fn send_phase1_request(&mut self, target: T::AcceptorId, t: T::Time);
    fn recv_phase1_reply(&mut self) -> (T::AcceptorId, (T::Time, Acceptor<T>));

    fn send_phase2_request(
        &mut self,
        target: T::AcceptorId,
        t: T::Time,
        proposal: Proposal<T, T::Part>,
    );
    fn recv_phase2_reply(&mut self) -> (T::AcceptorId, bool);
}

/// Abstract Paxos
pub struct APaxos<T: Types> {
    /// Acceptors stores value or part of the value that is proposed by a
    /// [`Proposer`].
    ///
    /// [`Proposer`]: crate::apaxos::proposer::Proposer
    acceptors: BTreeMap<T::AcceptorId, ()>,

    /// Quorum set defines quorums for read and write.
    ///
    /// A value that is accepted by a quorum is considered committed.
    quorum_set: T::QuorumSet,

    /// Transport for sending and receiving messages.
    transport: T::Transport,
}

impl<T: Types> APaxos<T> {
    pub fn new(
        acceptors: impl IntoIterator<Item = T::AcceptorId>,
        quorum_set: T::QuorumSet,
        transport: T::Transport,
    ) -> Self {
        let acceptors = acceptors.into_iter().map(|id| (id, ())).collect();

        Self {
            acceptors,
            quorum_set,
            transport,
        }
    }
}
