use std::fmt;
use std::fmt::Display;

use crate::Types;

#[derive(Debug, Clone, PartialEq, Eq, thiserror:Error)]
pub struct QuorumError<T: Types> {
    msg: String,
    quorum_set: String,
    received: Vec<T::AcceptorId>,
}

impl<T> fmt::Display for QuorumError<T>
where T: Types
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "QuorumError: {}: quorum_set: {}, received: {:?}",
            self.msg, self.quorum_set, self.received
        )
    }
}

impl<T: Types> QuorumError<T> {
    pub fn new(
        msg: impl Display,
        quorum_set: impl Display,
        received: impl IntoIterator<Item = T::AcceptorId>,
    ) -> Self {
        Self {
            msg: msg.to_string(),
            quorum_set: quorum_set.to_string(),
            received: received.into_iter().collect(),
        }
    }
}
