#![feature(step_trait)]

use std::{collections::HashMap, iter::Step};

trait Key: Eq + std::hash::Hash {}

impl<T> Key for T where T: Eq + std::hash::Hash {}

#[derive(Clone, Debug, Default)]
pub struct RollTable<K, V>
where
    K: Key,
{
    storage: HashMap<K, V>,
}

impl<K: Key, V> RollTable<K, V> {
    fn inner_mut(&mut self) -> &mut HashMap<K, V> {
        &mut self.storage
    }

    fn inner(&self) -> &HashMap<K, V> {
        &self.storage
    }
}

impl<K: Key, V> RollTable<K, V>
where
    K: Step,
    V: Clone,
{
    fn insert_range(&mut self, k: std::ops::Range<K>, v: V) {
        for key in k.collect::<Vec<_>>().into_iter() {
            self.storage.insert(key, v.clone());
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::RollTable;

    #[test]
    fn test_insert() {
        let mut table: RollTable<i32, i32> = RollTable::default();
        table.inner_mut().insert(12, 53);
        assert_eq!(table.inner().get(&12), Some(&53));
    }
}
