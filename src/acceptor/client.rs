#[cfg(doc)]
use crate::acceptor::Acceptor;
use crate::aliases::MonoHistories;
use crate::apaxos::history::mono_history::MonoHistory;
use crate::system::System;

/// A client to connect to a remote acceptor process.
///
/// This trait defines the interface for communicating with an acceptor in a
/// distributed system. Implementations of this trait handle the network
/// communication details.
pub trait Client<S: System> {
    type Error: std::error::Error + Send + Sync + 'static;

    /// Returns the ID of the acceptor this client connects to.
    fn id(&self) -> S::Types::AcceptorId;

    /// Reads all monotonic histories stored on the remote [`Acceptor`].
    async fn read(&self) -> Result<MonoHistories<S>, Self::Error>;

    /// Writes a monotonic history to the remote [`Acceptor`].
    async fn write(&self, history: S::MonoHistory) -> Result<(), Self::Error>;
}
