use gomoku_core::board::Board;
use iced::{
    widget::{container, Canvas, Column, Container, Row},
    Length,
};

use crate::{Message, Square};

pub fn get(board: Board) -> Container<'static, Message> {
    let mut rows = Row::new();

    for (y, row) in board.board.iter().enumerate() {
        let mut cols = Column::new();

        for (x, col) in row.iter().enumerate() {
            let color = if (y + x) % 2 == 0 {
                [255.0, 0.0, 0.0]
            } else {
                [255.0, 255.0, 0.0]
            };
            cols = cols.push(
                Canvas::new(Square::new(color))
                    .width(Length::Units(20))
                    .height(Length::Units(20)),
            );
        }

        rows = rows.push(cols);
    }

    container(rows)
}
