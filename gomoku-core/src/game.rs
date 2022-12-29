use crate::{ai::Ai, board::*, location::Location};

pub struct GameSettings {
    pub board_size: usize,
    pub ai1_starting: bool,
}

pub struct Game {
    ai1: Ai,
    ai2: Ai,
}

impl Game {
    pub fn init(path1: &str, path2: &str) -> Result<Self, String> {
        let ai1 = Ai::from_path(path1)?;
        let ai2 = Ai::from_path(path2)?;

        Ok(Self { ai1, ai2 })
    }

    fn tick(&mut self, turn: usize, board: &mut Board) {
        if turn % 2 == 0 {
            println!("Ai1's turn!");
            let action = self.ai1.read();
            match Location::from_string(action) {
                Ok(location) => {
                    board.place(location.x, location.y, Cell::Ai1);
                    Play::Turn(location).process(&mut self.ai2);
                }
                Err(err) => {
                    eprintln!("could not get location: [{}]", err);
                }
            }
        } else {
            println!("Ai2's turn!");
            let action = self.ai2.read();
            match Location::from_string(action) {
                Ok(location) => {
                    board.place(location.x, location.y, Cell::Ai2);
                    Play::Turn(location).process(&mut self.ai1);
                }
                Err(err) => {
                    eprintln!("could not get location: [{}]", err);
                }
            }
        }
        board.display();
    }

    /// Run game making AIs fight each other
    pub fn run(&mut self, settings: &GameSettings) {
        let mut board = Board::new(settings.board_size);
        let mut turn = 0;
        self.ai1.reset(settings.board_size);
        self.ai2.reset(settings.board_size);

        if settings.ai1_starting {
            Play::Begin.process(&mut self.ai1);
        } else {
            turn = 1;
            Play::Begin.process(&mut self.ai2);
        }

        loop {
            match board.check_win() {
                Some(cell) => {
                    if cell == Cell::Ai1 {
                        println!("Ai 1 has won");
                        break;
                    } else {
                        println!("Ai 2 has won");
                        break;
                    }
                }
                None => {
                    self.tick(turn, &mut board);
                    turn += 1;
                }
            }
        }

        self.ai1.stop();
        self.ai2.stop();
    }
}

pub enum Play {
    Turn(Location),
    Begin,
    Invalid,
}

impl Play {
    pub fn process(&self, ai: &mut Ai) {
        match self {
            Self::Turn(location) => {
                ai.write(format!("TURN {location}", location = location.to_string()));
                ai.flush();
            }
            Self::Begin => {
                ai.write("BEGIN".to_string());
                ai.flush();
            }
            Self::Invalid => {
                eprintln!("Invalid play !");
            }
        }
    }
}
