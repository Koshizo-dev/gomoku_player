use std::sync::Arc;

use gomoku_core::board::Board;
use iced::widget::Container;

use crate::Message;

mod board;

pub enum Entity {
    Board(Board),
}

impl Entity {
    pub fn get(self) -> Container<'static, Message> {
        match self {
            Self::Board(board) => board::get(board),
        }
    }
}
