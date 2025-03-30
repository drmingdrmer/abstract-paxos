use std::collections::HashSet;

use crate::apaxos::history::binary_compare::BinaryCompare;
use crate::apaxos::history::compatible::Compatible;
use crate::Types;

/// A history that contains at most one maximum time.
///
/// A [`MonoHistory`] has the following properties:
/// - It can determine if it is compatible with another history through the
///   [`Compatible`] trait.
/// - If two [`MonoHistory`]s are not compatible, they must be comparable via
///   the [`BinaryCompare`] trait.
pub trait MonoHistory<T: Types>
where
    Self: Compatible,
    Self: BinaryCompare,
{
    /// Return the maximum time of the history.
    ///
    /// There are at most one time in a [`MonoHistory`].
    fn max_time(&self) -> Option<T::Time>;

    /// Return the first found [`MonoHistory`] that is conflicting with this
    /// history and is greater than this history.
    ///
    /// If such a [`MonoHistory`] exists, the current history will be masked
    /// (not visible) to readers. This method is used to determine if this
    /// history is observable in a collection of histories.
    fn find_greater_conflict<'a>(&self, among: impl Iterator<Item = &'a Self>) -> Option<&'a Self> {
        for other in among {
            if self.is_compatible_with(other) {
                continue;
            }

            // conflict

            if self.is_less_than(other) {
                return Some(other);
            }
        }
        None
    }

    /// Observe a multiverse and collapse it into a universe, by eliminating
    /// conflicting histories.
    fn observe(mut monos: HashSet<Self>) -> HashSet<Self> {
        let mut observed = HashSet::new();

        while !monos.is_empty() {
            let mut candidate = monos.iter().next().unwrap();

            loop {
                if let Some(greater_conflict) = candidate.find_greater_conflict(monos.iter()) {
                    candidate = greater_conflict;
                } else {
                    let x = monos.remove(candidate);
                    observed.insert(x);

                    monos.retain(|mono| mono.is_compatible_with(candidate));
                    break;
                }
            }
        }
        observed
    }
}
