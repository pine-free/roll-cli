use rusty_dice::cards::Card;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Atom {
    RollResult(i32),
    Card(Card),
    Text(String),
}
