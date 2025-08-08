#![feature(step_trait)]

pub mod data;
pub use data::RollTable;

use thiserror::Error;

pub mod parse;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum RollTableError {
    #[error("failed to parse roll table")]
    ParsingError(String),
}

#[cfg(test)]
mod tests {
    use crate::data::RollTable;

    #[test]
    fn test_insert() {
        let mut table: RollTable<i32, i32> = RollTable::default();
        table.inner_mut().insert(12, 53);
        assert_eq!(table.inner().get(&12), Some(&53));
    }

    #[test]
    fn test_insert_range() {
        let mut table: RollTable<i32, i32> = RollTable::default();
        let val = 42;
        table.insert_iter(1..=10, val);
        for k in 1..=10 {
            assert_eq!(table.inner().get(&k), Some(&val));
        }
    }
}
