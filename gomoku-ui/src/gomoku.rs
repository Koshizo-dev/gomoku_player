use gomoku_core::board::{Board, Cell};
use gomoku_core::location::Location;
use iced::executor;
use iced::widget::{container, Column};
use iced::{Application, Command, Element, Length, Settings, Theme};

use crate::{Entity, Message};

pub struct Gomoku {
    board: Board,
    thinking: Option<usize>,
    actions: Vec<(Location, Cell)>,
}

impl Gomoku {
    pub fn run() {
        let _ = <Gomoku as Application>::run(Settings::default());
    }
}

impl Application for Gomoku {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Gomoku, Command<Message>) {
        (
            Gomoku {
                board: Board::new(20),
                thinking: None,
                actions: Vec::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Gomoku player")
    }

    fn update(&mut self, _message: Message) -> Command<Message> {
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let board = Entity::Board(self.board.clone()).get();

        container(board)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .padding(20)
            .into()
    }
}
