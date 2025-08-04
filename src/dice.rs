use rusty_dice::Dice;

#[derive(Debug, PartialEq, Clone)]
pub struct Calculation {
    pub dice: Vec<Dice>,
    pub numbers: Vec<u32>,
}

impl Calculation {
    pub fn new(dice: &[Dice], nums: &[u32]) -> Self {
        Self {
            dice: Vec::from(dice),
            numbers: Vec::from(nums),
        }
    }

    /// Does not give info on what values were produced, just gives the sum
    pub fn roll(&self) -> u32 {
        let vals = self.dice.iter().map(Dice::roll).flatten().sum::<u32>();
        let nums_total = self.numbers.iter().sum::<u32>();

        vals + nums_total
    }
}

fn plus_join(i: &Vec<impl ToString>) -> String {
    i.iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(" + ")
}

impl ToString for Calculation {
    fn to_string(&self) -> String {
        let dice_str = plus_join(&self.dice);
        let nums_str = if self.numbers.is_empty() {
            String::new()
        } else {
            plus_join(&self.numbers)
        };

        if nums_str.is_empty() {
            dice_str
        } else {
            format!("{} + {}", dice_str, nums_str)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RollResults {
    pub name: String,
    pub results: Vec<u32>,
}

impl RollResults {
    fn new(name: &impl ToString, results: &[u32]) -> Self {
        Self {
            name: name.to_string(),
            results: Vec::from(results),
        }
    }
}

impl From<&Dice> for RollResults {
    fn from(value: &Dice) -> Self {
        Self::new(value, &value.roll())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RollExpression {
    pub name: Option<String>,
    pub calculation: Calculation,
}

impl RollExpression {
    pub fn new(name: &str, calculation: &Calculation) -> Self {
        Self {
            name: Some(name.to_string()),
            calculation: calculation.clone(),
        }
    }

    pub fn nameless(calculation: &Calculation) -> Self {
        Self {
            name: None,
            calculation: calculation.clone(),
        }
    }
}

impl ToString for RollExpression {
    fn to_string(&self) -> String {
        if let Some(n) = &self.name {
            n.clone()
        } else {
            self.calculation.to_string()
        }
    }
}
