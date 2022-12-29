use iced::executor;
use iced::widget::{container, Column};
use iced::{Application, Command, Element, Length, Settings, Theme};

pub struct Gomoku {}

#[derive(Debug)]
pub enum GameUpdate {
    Board(),
}

#[derive(Debug)]
pub enum Message {
    GameUpdate(GameUpdate),
    UpdateView,
    Previous,
    Next,
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
        (Gomoku {}, Command::none())
    }

    fn title(&self) -> String {
        String::from("Gomoku player")
    }

    fn update(&mut self, _message: Message) -> Command<Message> {
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        container(Column::new())
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .padding(20)
            .into()
    }
}
