use super::RollResults;

/// Trait describing functions that can be used as modifiers
/// for dice roll results
///
/// This trait is implemented for closures, and you can
/// define your own modifiers by implementing it
pub trait RollModifier {
    /// The output of the modifier
    ///
    /// Typically this is RollResults, but other variants are supported
    type Output;

    /// The method that modifies the results
    fn apply(&self, input: RollResults) -> Self::Output;
}

/// A common kind of RollModifier that one set of values
/// of a roll to another one
pub trait RollMapping: RollModifier {
    /// Map the values
    fn map(self, input: RollResults) -> RollResults;
}

impl<M> RollMapping for M
where
    M: RollModifier<Output = RollResults>,
{
    fn map(self, input: RollResults) -> RollResults {
        self.apply(input)
    }
}

/// Keep n highest dice
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct KeepHighest(pub usize);

impl std::fmt::Display for KeepHighest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "kh{}", self.0)
    }
}

impl RollModifier for KeepHighest {
    type Output = RollResults;

    fn apply(&self, input: RollResults) -> Self::Output {
        let n_skip = input.len().saturating_sub(self.0);
        input.into_iter().skip(n_skip).collect()
    }
}

/// Drop n lowest dice
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DropLowest(pub usize);

impl std::fmt::Display for DropLowest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "dl{}", self.0)
    }
}

impl RollModifier for DropLowest {
    type Output = RollResults;

    fn apply(&self, input: RollResults) -> Self::Output {
        let keep = KeepHighest(input.len() - self.0);
        keep.apply(input)
    }
}

/// Keep n lowest dice
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct KeepLowest(pub usize);

impl RollModifier for KeepLowest {
    type Output = RollResults;

    fn apply(&self, input: RollResults) -> Self::Output {
        input.into_iter().take(self.0).collect()
    }
}

impl std::fmt::Display for KeepLowest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "kl{}", self.0)
    }
}

/// Drop n highest dice
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DropHighest(pub usize);

impl std::fmt::Display for DropHighest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "dh{}", self.0)
    }
}

impl RollModifier for DropHighest {
    type Output = RollResults;

    fn apply(&self, input: RollResults) -> Self::Output {
        let keep = KeepLowest(input.len() - self.0);
        keep.apply(input)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// An enumeration of built-in roll modifiers
pub enum RollModifiers {
    /// See inner struct documentation
    KeepLowest(KeepLowest),

    /// See inner struct documentation
    KeepHighest(KeepHighest),

    /// See inner struct documentation
    DropLowest(DropLowest),

    /// See inner struct documentation
    DropHighest(DropHighest),
}

/// Modifier that can be displayed
pub trait DisplayableModifier: RollMapping<Output = Vec<u32>> + std::fmt::Display {}

impl<T> DisplayableModifier for T where T: RollMapping<Output = Vec<u32>> + std::fmt::Display {}

impl RollModifiers {
    /// Provides access to the inner roll modifier object
    pub fn inner(&self) -> Box<dyn DisplayableModifier> {
        match self {
            RollModifiers::KeepLowest(i) => Box::new(*i),
            RollModifiers::KeepHighest(i) => Box::new(*i),
            RollModifiers::DropLowest(i) => Box::new(*i),
            RollModifiers::DropHighest(i) => Box::new(*i),
        }
    }
}

impl std::fmt::Display for RollModifiers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let repr = self.inner().to_string();
        write!(f, "{repr}")
    }
}
