use std::env;

use gomoku_player::{
    game::{Game, GameSettings},
    test::Test,
};

enum StartMode {
    Fight(String, String),
    Test(String),
    Unknown,
    Incorrect,
}

impl StartMode {
    pub fn exec(&self) {
        match self {
            Self::Unknown => {
                show_help();
                panic!("Unknown mode");
            }
            Self::Incorrect => {}
            Self::Fight(ai1_path, ai2_path) => {
                println!("Running in fight mode!");
                Game::init(ai1_path, ai2_path)
                    .expect("")
                    .run(&GameSettings {
                        board_size: 20,
                        ai1_starting: true,
                    });
            }
            Self::Test(ai_path) => {
                println!("Running in test mode!");
                Test::init(ai_path).expect("").run();
            }
        }
    }
}

fn show_help() {
    let mut help = String::new();

    let messages = vec![
        "Gomoku player, made by RqndomHax in Rust",
        "",
        "Usage: gomoku_player [COMMAND] [ARGS]",
        "",
        "Commands:",
        "\t--test <AI_PATH>\t\tRun functionnal tests for <AI_PATH>",
        "\t--fight <AI1_PATH> <AI2_PATH>\t\tRun a fight between <AI1_PATH> and <AI2_PATH>",
    ];

    for message in messages {
        help += &format!("{}\n", message);
    }

    print!("{}", help)
}

fn check_args(args: Vec<String>) -> StartMode {
    match args[0].as_str() {
        "--test" => {
            if args.len() != 2 {
                println!("<AI_PATH> expected!");
                return StartMode::Incorrect;
            }

            StartMode::Test(args[1].clone())
        }
        "--fight" => {
            if args.len() != 3 {
                println!("<AI_PATH> missing!");
                return StartMode::Incorrect;
            }

            StartMode::Fight(args[1].clone(), args[2].clone())
        }
        _ => StartMode::Unknown,
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() == 0 {
        return show_help();
    }

    check_args(args).exec();
}
