use std::io::{Write, stdout};

use crate::{
    game::game::Game,
    types::{ColourConfig, ConsoleBackgroundColour, GameResult, GameState},
};
use crate::types::LetterState;
pub struct App {
    pub game: Game,
    pub colour_config: ColourConfig,
}
impl App {
    pub fn new(difficulty: &crate::types::GameDifficulty) -> Self {
        let colour_config = super::config::load_color_config();
        return App {
            game: Game::new(None, Some(*difficulty), None),
            colour_config: colour_config,
        };
    }
    pub fn start(&mut self) {
        self.game.start();
        println!(
            "Welcome to Wordle! Word length is {}. Good Luck!",
            ConsoleBackgroundColour::Red.colour_text(self.game.get_word_length().to_string().as_str()),
        );
        while self.game.get_state() == GameState::InProgress {
            print!(
                "{}/{} Enter your guess: ",
                self.game.get_attempts() + 1,
                self.game.get_max_attempts()
            );
            stdout().flush().expect("Failed to flush.");
            let mut input = String::new();
            std::io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            let input = input.trim();
            match self.game.guess(input) {
                Ok(result) => {
                    for i in 0..input.len() {
                        match result[i] {
                            LetterState::Matched => print!("{}", self.colour_config[0].colour_text(input.chars().nth(i).unwrap().to_string().as_str())),
                            LetterState::Present => print!("{}", self.colour_config[1].colour_text(input.chars().nth(i).unwrap().to_string().as_str())),
                            LetterState::Absent => print!("{}", self.colour_config[2].colour_text(input.chars().nth(i).unwrap().to_string().as_str())),
                        }
                    }
                }
                Err(err) => {
                    println!("{}", err);
                }
            }
            println!();
        }
        match self.game.get_state() {
            GameState::Over(result) => match result {
                GameResult::Won => println!("Congratulations! You won!"),
                GameResult::Lost => println!(
                    "Game over! The correct answer was: {}",
                    self.game.get_answer()
                ),
            },
            _ => {}
        }
    }
}
