use std::{collections::HashMap, iter::Step};

pub(crate) trait Key: Eq + std::hash::Hash {}

impl<T> Key for T where T: Eq + std::hash::Hash {}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct RollTable<K, V>
where
    K: Key,
{
    pub(crate) storage: HashMap<K, V>,
}

impl<K: Key, V> RollTable<K, V> {
    pub fn new(storage: HashMap<K, V>) -> Self {
        Self { storage }
    }

    pub(crate) fn inner_mut(&mut self) -> &mut HashMap<K, V> {
        &mut self.storage
    }

    pub(crate) fn inner(&self) -> &HashMap<K, V> {
        &self.storage
    }
}

impl<K: Key, V> RollTable<K, V>
where
    K: Step,
    V: Clone,
{
    pub(crate) fn insert_iter<I>(&mut self, k: I, v: V)
    where
        I: Iterator<Item = K>,
    {
        for key in k.collect::<Vec<_>>().into_iter() {
            self.storage.insert(key, v.clone());
        }
    }
}
