use gomoku_player::{Game, GameSettings};

fn main() {
    let settings = GameSettings {
        board_size: 20,
        ai1_starting: true,
    };
    let game = Game::init(
        "../B-AIA-500-MPL-5-1-gomoku-mickael.grezes/pbrain-gomoku-ai",
        "../B-AIA-500-MPL-5-1-gomoku-mickael.grezes/pbrain-gomoku-ai",
    );

    let mut game = match game {
        Ok(game) => game,
        Err(err) => {
            eprintln!("Error whilst initializing Game: [{}]", err);
            return;
        }
    };

    // game.run(&settings);
    game.run_tests();
}
