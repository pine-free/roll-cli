use rand::Rng;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Dice {
    pub num_sides: u32,
    pub quantity: u32,
}

impl Dice {
    pub fn roll(&self) -> Vec<u32> {
        (1..=self.quantity)
            .map(|_| rand::thread_rng().gen_range(1..=self.num_sides))
            .collect()
    }
}

impl ToString for Dice {
    fn to_string(&self) -> String {
        format!("{}d{}", self.quantity, self.num_sides)
    }
}
