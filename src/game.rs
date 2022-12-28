use crate::{Ai, Board, Cell, Runtime};

pub struct TestCondition {
    board: Vec<(Location, Cell)>,
    expected_moves: Vec<Location>,
    result_board: Option<Board>,
}

pub enum TestResult {
    Success(Location),
    Fail(Location, Board),
    Error,
}

impl TestCondition {
    pub fn new(board: Vec<(Location, Cell)>, expected_moves: Vec<Location>) -> Self {
        Self {
            board,
            expected_moves,
            result_board: None,
        }
    }

    pub fn send_board(&mut self, ai: &mut Ai) {
        let mut board = Board::new(20);
        ai.reset(20);
        ai.write("BOARD".to_string());
        for cell in &self.board {
            board.place(cell.0.x, cell.0.y, cell.1);
            ai.write(format!("{},{}", cell.0.to_string(), cell.1.get_id()));
        }
        ai.write("DONE".to_string());
        self.result_board = Some(board);
    }

    pub fn read_move(&mut self, ai: &mut Ai) -> TestResult {
        ai.flush();
        let action = Location::from_string(ai.read());

        let action = match action {
            Ok(action) => action,
            Err(err) => {
                eprintln!("Error whilst converting move: [{}]", err);
                return TestResult::Error;
            }
        };

        let mut result_board = self.result_board.clone().unwrap_or(Board::new(20));

        result_board.place(action.x, action.y, Cell::NewAi1);

        if self.expected_moves.contains(&action) {
            TestResult::Success(action)
        } else {
            TestResult::Fail(action, result_board)
        }
    }
}

pub struct GameSettings {
    pub board_size: usize,
    pub ai1_starting: bool,
}

pub struct Game {
    ai1: Ai,
    ai2: Ai,
}

impl Game {
    fn get_ai(path: &str) -> Result<Ai, String> {
        let runtime = Runtime::init(path)?;

        Ok(Ai::new(runtime))
    }

    pub fn init(path1: &str, path2: &str) -> Result<Self, String> {
        let ai1 = Self::get_ai(path1)?;
        let ai2 = Self::get_ai(path2)?;

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

    fn start_tests(&mut self, name: &str, mut tests: Vec<TestCondition>) {
        for (id, test) in tests.iter_mut().enumerate() {
            test.send_board(&mut self.ai1);
            println!("{}_test-{}: [{}]", name, id, {
                match test.read_move(&mut self.ai1) {
                    TestResult::Success(location) => format!("success ({})", location.to_string()),
                    TestResult::Fail(location, board) => {
                        board.display();
                        format!("failed ({})", location.to_string())
                    }
                    TestResult::Error => "failed".to_string(),
                }
            });
        }
    }

    fn row_tests(&mut self, cell: Cell) {
        let tests = vec![
            TestCondition::new(
                vec![
                    ((2, 3).into(), cell),
                    ((3, 3).into(), cell),
                    ((4, 3).into(), cell),
                    ((6, 3).into(), cell),
                ],
                vec![(5, 3).into()],
            ),
            TestCondition::new(
                vec![
                    ((2, 3).into(), cell),
                    ((3, 3).into(), cell),
                    ((5, 3).into(), cell),
                    ((6, 3).into(), cell),
                ],
                vec![(4, 3).into()],
            ),
            TestCondition::new(
                vec![
                    ((2, 3).into(), cell),
                    ((4, 3).into(), cell),
                    ((5, 3).into(), cell),
                    ((6, 3).into(), cell),
                ],
                vec![(3, 3).into()],
            ),
        ];

        self.start_tests("col", tests);
    }

    fn col_tests(&mut self, cell: Cell) {
        let tests = vec![
            TestCondition::new(
                vec![
                    ((3, 2).into(), cell),
                    ((3, 3).into(), cell),
                    ((3, 4).into(), cell),
                    ((3, 6).into(), cell),
                ],
                vec![(3, 5).into()],
            ),
            TestCondition::new(
                vec![
                    ((3, 2).into(), cell),
                    ((3, 3).into(), cell),
                    ((3, 5).into(), cell),
                    ((3, 6).into(), cell),
                ],
                vec![(3, 4).into()],
            ),
            TestCondition::new(
                vec![
                    ((3, 2).into(), cell),
                    ((3, 4).into(), cell),
                    ((3, 5).into(), cell),
                    ((3, 6).into(), cell),
                ],
                vec![(3, 3).into()],
            ),
        ];

        self.start_tests("row", tests);
    }

    /// Tests will be ran on `ai1`
    pub fn run_tests(&mut self) {
        // check defense
        self.col_tests(Cell::Ai2);
        self.row_tests(Cell::Ai2);

        // check attack

        self.ai1.stop();
        self.ai2.stop();
    }
}

#[derive(PartialEq)]
pub struct Location {
    pub x: usize,
    pub y: usize,
}

impl From<(usize, usize)> for Location {
    fn from(tuple: (usize, usize)) -> Self {
        Location {
            x: tuple.0,
            y: tuple.1,
        }
    }
}

impl Location {
    pub fn to_string(&self) -> String {
        format!("{x},{y}", x = self.x, y = self.y)
    }

    pub fn from_string(target: String) -> Result<Location, String> {
        let target = target.trim_end_matches('\n');
        let coords: Vec<&str> = target.split(',').collect();

        if coords.len() != 2 {
            return Err("Wrong number of arguments in coords".to_string());
        }

        let x: usize = match coords[0].parse() {
            Ok(x) => x,
            Err(err) => {
                return Err(err.to_string());
            }
        };

        let y: usize = match coords[1].parse() {
            Ok(y) => y,
            Err(err) => {
                return Err(err.to_string());
            }
        };

        Ok(Location { x, y })
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
