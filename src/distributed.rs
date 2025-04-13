use futures::StreamExt;
use futures::stream;
use log::error;

use crate::acceptor;
use crate::acceptor::Client;
use crate::aliases::MonoHistories;
use crate::errors::QuorumError;
use crate::quorum_set::QuorumSet;
use crate::system::System;

pub trait Distributed<S: System> {
    type Client: acceptor::Client<S>;

    type QuorumSet: QuorumSet<S::Types>;

    fn quorum_set(&self) -> &Self::QuorumSet;

    fn get_client(&self, id: &S::Types::AcceptorId) -> Self::Client;

    fn read(&self) -> Result<MonoHistories<S>, QuorumError<S::Types>> {
        let targets = self.quorum_set().get_read_quorum();
        self.read_from_acceptors(&targets)
    }

    async fn read_from_acceptors(
        &self,
        targets: &[S::Types::AcceptorId],
    ) -> Result<MonoHistories<S>, QuorumError<S::Types>> {
        // pending RPCs
        let mut pending = stream::FuturesUnordered::new();

        let mut all_monos = MonoHistories::default();
        let mut received = vec![];

        for acceptor_id in targets {
            let client = self.get_client(acceptor_id);

            let fu = async move {
                let res = client.read().await;
                (client.id(), res)
            };

            pending.push(fu);
        }

        let quorum_set = self.quorum_set();

        while let Some((id, res)) = pending.next().await {
            match res {
                Ok(monos) => {
                    received.push(id);
                    all_monos.extend(monos);

                    if quorum_set.is_read_quorum(&received) {
                        break;
                    }
                }
                Err(e) => {
                    error!("Error reading from acceptor {}: {}", id, e);
                }
            }
        }

        Err(QuorumError::new("p0-read fail", quorum_set, received))
    }

    async fn write_to_acceptors(&mut self, targets: &[S::Types::AcceptorId], h: S::MonoHistory) {
        // pending RPCs
        let mut pending = stream::FuturesUnordered::new();

        let mut received = vec![];

        for acceptor_id in targets {
            let client = self.get_client(acceptor_id);

            let fu = async move {
                let res = client.write(h.clone()).await;
                (client.id(), res)
            };

            pending.push(fu);
        }

        let quorum_set = self.quorum_set();

        while let Some((id, res)) = pending.next().await {
            match res {
                Ok(_) => {
                    received.push(id);

                    if quorum_set.is_read_quorum(&received) {
                        break;
                    }
                }
                Err(e) => {
                    error!("Error reading from acceptor {}: {}", id, e);
                }
            }
        }

        Err(QuorumError::new(quorum_set, received))
    }
}
