#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Player {
    None,
    X,
    O,
}

impl From<Player> for String {
    fn from(player: Player) -> Self {
        match player {
            Player::X => "X",
            Player::O => "O",
            Player::None => ".",
        }
        .into()
    }
}

impl From<char> for Player {
    fn from(c: char) -> Self {
        match c {
            'X' | 'x' => Player::X,
            'O' | 'o' | '0' => Player::O,
            '.' => Player::None,
            _ => panic!(),
        }
        .into()
    }
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
