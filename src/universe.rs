use crate::aliases::MonoHistories;
use crate::apaxos::history::mono_history::MonoHistory;
use crate::distributed::Distributed;
use crate::errors::QuorumError;
use crate::system::System;

pub trait Universe<S: System>
where Self: Distributed<S>
{
    fn read(&self) -> Result<MonoHistories<S>, QuorumError<S::Types>> {
        let monos = Distributed::read(self)?;
        let observed = S::MonoHistory::observe(monos);
        Ok(observed)
    }
}
