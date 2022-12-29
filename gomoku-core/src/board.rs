use std::fmt::{Display, Formatter, Result};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Cell {
    Empty,
    Ai1,
    Ai2,
    NewAi1,
}

impl Cell {
    pub fn get_id(&self) -> usize {
        match self {
            Cell::Ai1 => 1,
            Cell::Ai2 => 2,
            _ => 0,
        }
    }
}

impl From<usize> for Cell {
    fn from(cell_type: usize) -> Self {
        match cell_type {
            1 => Self::Ai1,
            2 => Self::Ai2,
            _ => Self::Empty,
        }
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let symbol = match self {
            Cell::Empty => "-",
            Cell::Ai1 => "o",
            Cell::NewAi1 => "p",
            Cell::Ai2 => "x",
        };
        write!(f, "{}", symbol)
    }
}

#[derive(Clone, Debug)]
pub struct Board {
    pub board: Vec<Vec<Cell>>,
    pub size: usize,
}

impl Board {
    pub fn new(size: usize) -> Self {
        let mut rows: Vec<Vec<Cell>> = Vec::new();
        rows.reserve(size);
        for _ in 0..size {
            let cols = vec![Cell::Empty; size];
            rows.push(cols);
        }

        Self { board: rows, size }
    }

    pub fn place(&mut self, x: usize, y: usize, cell: Cell) {
        if self.board[y][x] != Cell::Empty {
            println!(
                "ILLEGAL MOVE AT (x,y): ({},{}): ({})",
                x, y, self.board[y][x]
            );
            println!(
                "ILLEGAL MOVE AT (x,y): ({},{}): ({})",
                x, y, self.board[y][x]
            );
            println!(
                "ILLEGAL MOVE AT (x,y): ({},{}): ({})",
                x, y, self.board[y][x]
            );
        }
        self.board[y][x] = cell;
    }

    pub fn display(&self) {
        // Calculate the number of digits in the largest index
        let max_digits = self.size.to_string().len();

        // Print column indices
        print!("\t");
        for i in 0..self.size {
            let spaces = " ".repeat(max_digits - i.to_string().len() + 1);
            print!("{}{}", i, spaces);
        }
        println!("");

        // Print rows
        for (i, row) in self.board.iter().enumerate() {
            let spaces = " ".repeat(max_digits - i.to_string().len());
            print!("{}{}\t", i, spaces);
            for cell in row.iter() {
                let spaces = " ".repeat(max_digits - cell.to_string().len() + 1);
                print!("{}{}", cell, spaces);
            }
            println!("");
        }
    }

    fn check_pattern(&self, pattern: [Cell; 5]) -> bool {
        let pattern_size = pattern.len();

        // Check rows
        for row in &self.board {
            for i in 0..self.size - self.size + 1 {
                if &row[i..i + self.size] == pattern {
                    return true;
                }
            }
        }

        // Check columns
        for col in 0..self.size {
            let mut col_values = Vec::new();
            for row in &self.board {
                col_values.push(row[col]);
            }
            for i in 0..self.size - pattern_size + 1 {
                if &col_values[i..i + pattern_size] == pattern {
                    return true;
                }
            }
        }

        // Check diagonals
        for i in 0..self.size - pattern_size + 1 {
            for j in 0..self.size - pattern_size + 1 {
                let mut diag_values = Vec::new();
                for k in 0..pattern_size {
                    diag_values.push(self.board[i + k][j + k]);
                }
                if diag_values == pattern {
                    return true;
                }
            }
        }
        for i in (pattern_size - 1)..self.size {
            for j in 0..self.size - pattern_size + 1 {
                let mut diag_values = Vec::new();
                for k in 0..pattern_size {
                    diag_values.push(self.board[i - k][j + k]);
                }
                if diag_values == pattern {
                    return true;
                }
            }
        }
        return false;
    }

    pub fn check_win(&self) -> Option<Cell> {
        if self.check_pattern([Cell::Ai1, Cell::Ai1, Cell::Ai1, Cell::Ai1, Cell::Ai1]) {
            return Some(Cell::Ai1);
        }
        if self.check_pattern([Cell::Ai2, Cell::Ai2, Cell::Ai2, Cell::Ai2, Cell::Ai2]) {
            return Some(Cell::Ai2);
        }
        None
    }
}
