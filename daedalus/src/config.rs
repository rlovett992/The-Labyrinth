#[derive(Debug, Clone, Copy)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
    Labyrinthian
}

impl Difficulty {
    pub fn random_size(self) -> usize {
        use rand::Rng;

        let mut rng = rand::rng();

        match self {
            Difficulty::Easy => rng.random_range(20..=49),
            Difficulty::Medium => rng.random_range(50..=99),
            Difficulty::Hard => rng.random_range(100..=249),
            Difficulty::Labyrinthian => rng.random_range(250..=1000),
        }
    }
}