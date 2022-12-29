use gomoku_core::{board::Cell, location::Location};

#[derive(Debug)]
pub enum GameUpdate {
    PlaceCell(Location, Cell),
    Thinking(usize),
}

#[derive(Debug)]
pub enum SettingsUpdate {
    BoardSize(usize),
}

#[derive(Debug)]
pub enum Message {
    GameUpdate(GameUpdate),
    SettingsUpdate(SettingsUpdate),
}
