use std::error::Error;
use std::fmt::Debug;
use std::fmt::Formatter;

use validit::Validate;

use crate::apaxos::accepted::Accepted;
use crate::apaxos::greater_equal::GreaterEqual;
use crate::apaxos::greater_equal_map::Map;
use crate::apaxos::proposal::Proposal;
use crate::Types;

#[derive(Clone)]
pub struct Acceptor<T: Types> {
    /// The key is the [`Time`] of the [`Proposer`] that sent the request.
    /// The value is a [`Fragment`] of the proposed value
    ///
    /// The **maximal** is the last time it has seen so far, i.e., the current
    /// time.
    pub store: Map<T::Time, Option<T::Part>>,

    /// The time it has seen so far, i.e., the current time.
    #[deprecated(note = "use `store` instead")]
    pub time: T::Time,

    /// The state that is accepted by this [`Acceptor`].
    #[deprecated(note = "use `store` instead")]
    pub accepted: Option<Accepted<T>>,
}

// TODO: use Valid<Acceptor<T>> instead of Acceptor<T>
impl<T: Types> Validate for Acceptor<T> {
    fn validate(&self) -> Result<(), Box<dyn Error>> {
        // TODO: valid time chain
        Ok(())
    }
}

impl<T: Types> Debug for Acceptor<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Acceptor").field("store", &self.store).finish()
    }
}

impl<T: Types> Default for Acceptor<T>
where T::Time: std::hash::Hash
{
    fn default() -> Self {
        Self {
            store: Map::new(),
            time: T::Time::default(),
            accepted: None,
        }
    }
}

impl<T: Types> Acceptor<T> {
    /// Handle the phase-1 request from a [`Proposer`], i.e., set up a new
    /// [`Time`] point.
    ///
    /// Returns the `Time` before handling the request and the updated
    /// [`Acceptor`] itself.
    ///
    /// The returned `Time` will be used to revert the `Time` if the
    /// [`Proposer`] decide to cancel this round of consensus algorithm.
    /// For example, **2PC** will revert the `Time` if the coordinator receives
    /// conflicting votes(otherwise other [`Proposer`] can not proceed). But
    /// **Classic Paxos** does not have to revert the `Time` but it could.
    pub(crate) fn handle_phase1_request(&mut self, now: T::Time) -> (T::Time, Self) {
        dbg!("handle_phase1_request", now, &self.store);
        dbg!(self.store.is_maximal(&now));

        if self.store.is_maximal(&now) {
            let _ = self.store.try_insert(now, None);
        }

        (now, self.clone())
    }

    /// Revert the `Time` if it is still a **maximal**.
    ///
    /// The proposer sending phase1-revert request must ensure no phase-2 is
    /// sent, otherwise consensus is not guaranteed.
    ///
    /// It returns a `bool` indicating whether the time is reverted.
    pub(crate) fn handle_phase1_revert_request(&mut self, t: T::Time) -> bool {
        let maximals = self.store.maximals().collect::<Vec<_>>();
        dbg!("handle_phase1_revert_request", t, maximals);

        // Revert the time to a previous one if it is still the same
        if self.store.is_maximal(&t) {
            let _ = self.store.remove(&t);
            // Reverting it twice, it is still valid.
            true
        } else {
            // There is some Time that is greater than `t`, can not revert.
            false
        }
    }

    pub(crate) fn handle_phase2_request(&mut self, data: Map<T::Time, Option<T::Part>>) -> bool {
        dbg!("handle_phase2_request", data);

        let maximals = data.maximals().collect::<Vec<_>>();
        dbg!("handle_phase2_request", maximals);

        assert_eq!(maximals.len(), 1);

        let t = maximals[0].0;

        if self.store.is_maximal(&t) {
            self.store.merge(data);
            true
        } else {
            false
        }
    }
}
