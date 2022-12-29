use crate::{
    ai::Ai,
    board::{Board, Cell},
    location::Location,
};

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

    fn send_board(&mut self, ai: &mut Ai) {
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

    fn read_move(&mut self, ai: &mut Ai) -> TestResult {
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

pub struct Test {
    ai: Ai,
}

impl Test {
    pub fn init(path: &str) -> Result<Self, String> {
        let ai = Ai::from_path(path)?;

        Ok(Self { ai })
    }

    pub fn start_tests(&mut self, name: &str, mut tests: Vec<TestCondition>) {
        for (id, test) in tests.iter_mut().enumerate() {
            test.send_board(&mut self.ai);
            println!("{}_test-{}: [{}]", name, id, {
                match test.read_move(&mut self.ai) {
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

    fn diag1_tests(&mut self, cell: Cell) {
        let tests = vec![
            TestCondition::new(
                vec![
                    ((2, 3).into(), cell),
                    ((3, 4).into(), cell),
                    ((4, 5).into(), cell),
                    ((6, 7).into(), cell),
                ],
                vec![(5, 6).into()],
            ),
            TestCondition::new(
                vec![
                    ((2, 3).into(), cell),
                    ((4, 5).into(), cell),
                    ((5, 6).into(), cell),
                    ((6, 7).into(), cell),
                ],
                vec![(3, 4).into()],
            ),
            TestCondition::new(
                vec![
                    ((2, 3).into(), cell),
                    ((3, 4).into(), cell),
                    ((5, 6).into(), cell),
                    ((6, 7).into(), cell),
                ],
                vec![(4, 5).into()],
            ),
        ];

        self.start_tests("diag1", tests);
    }

    fn diag2_tests(&mut self, cell: Cell) {
        let tests = vec![
            TestCondition::new(
                vec![
                    ((2, 7).into(), cell),
                    ((3, 6).into(), cell),
                    ((4, 5).into(), cell),
                    ((6, 3).into(), cell),
                ],
                vec![(5, 4).into()],
            ),
            TestCondition::new(
                vec![
                    ((2, 7).into(), cell),
                    ((4, 5).into(), cell),
                    ((5, 4).into(), cell),
                    ((6, 3).into(), cell),
                ],
                vec![(3, 6).into()],
            ),
            TestCondition::new(
                vec![
                    ((2, 7).into(), cell),
                    ((3, 6).into(), cell),
                    ((5, 4).into(), cell),
                    ((6, 3).into(), cell),
                ],
                vec![(4, 5).into()],
            ),
        ];

        self.start_tests("diag2", tests);
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
    pub fn run(&mut self) {
        let cells = vec![Cell::Ai1, Cell::Ai2];
        for cell in cells {
            match cell {
                Cell::Ai1 => println!("ATTACKING TEST MODE!"),
                _ => println!("DEFENDING TEST MODE!"),
            }
            self.col_tests(cell);
            self.row_tests(cell);
            self.diag1_tests(cell);
            self.diag2_tests(cell);
        }

        // After every tests we can stop the Ai
        self.ai.stop();
    }
}
