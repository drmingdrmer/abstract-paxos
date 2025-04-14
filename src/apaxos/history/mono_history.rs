use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;

use crate::Types;
use crate::aliases::MonoHistories;
use crate::apaxos::history::binary_compare::BinaryCompare;
use crate::apaxos::history::compatible::Compatible;

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
    fn observe(mut monos: HashMap<T::Time, Self>) -> HashMap<T::Time, Self> {
        let mut observed = HashMap::new();

        while !monos.is_empty() {
            let (_time, mut candidate) = monos.iter().next().unwrap();

            loop {
                if let Some(greater_conflict) = candidate.find_greater_conflict(monos.values()) {
                    candidate = greater_conflict;
                } else {
                    // Safe unwrap: it's greater than another thus it can not be empty.
                    let t = candidate.max_time().unwrap();
                    let x = monos.remove(&t);
                    observed.insert(t, x);

                    monos.retain(|t, mono| mono.is_compatible_with(candidate));
                    break;
                }
            }
        }
        observed
    }
}
