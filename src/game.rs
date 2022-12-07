#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Player {
    X,
    O,
    None,
}

impl Player {
    pub fn next(&self) -> Player {
        match self {
            Player::X => Player::O,
            Player::O => Player::X,
            Player::None => Player::None,
        }
    }
}

pub type Move = (usize, usize); // TODO revisit types with performance benchmarks
