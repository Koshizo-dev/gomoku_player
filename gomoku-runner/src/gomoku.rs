use std::env;

use gomoku_ui::Gomoku;

use gomoku_core::test::Test;

enum StartMode {
    Fight,
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
            Self::Fight => {
                println!("Running in fight mode!");
                Gomoku::run();
                //Game::init(ai1_path, ai2_path)
                //    .expect("")
                //    .run(&GameSettings {
                //        board_size: 20,
                //        ai1_starting: true,
                //    });
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
        "\t--fight\t\tRun the fight ui",
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
        "--fight" => StartMode::Fight,
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
