use indexmap::{Equivalent, IndexMap};
use std::hash::Hash;

/// Extension methods for [`IndexMap`]
pub(crate) trait IndexMapExt<K, V> {
    fn replace<T>(&mut self, from: &T, to: K) -> Option<usize>
    where
        T: Hash + Equivalent<K>;
}

impl<K, V> IndexMapExt<K, V> for IndexMap<K, V>
where
    K: Hash + Eq,
{
    fn replace<T: ?Sized>(&mut self, from: &T, to: K) -> Option<usize>
    where
        T: Hash + Equivalent<K>,
    {
        let (b, _, value) = self.swap_remove_full(from)?;
        let (a, _) = self.insert_full(to, value);
        self.swap_indices(a, b);
        Some(b)
    }
}
