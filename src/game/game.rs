use crate::{types::*, word::word::Word};
pub struct Game {
    state: GameState,
    word_length: usize,
    max_attempts: u8,
    guess_count: u8,
    answer: String,
    dictionary: Word,
}

impl Game {
    pub fn new(
        answer: Option<String>,
        difficulty: Option<GameDifficulty>,
        max_attempts: Option<u8>,
    ) -> Self {
        let difficulty = difficulty.unwrap_or(GameDifficulty::Medium);
        let dictionary = Word::new(&difficulty);
        let answer = match answer {
            Some(a) => a,
            None => {
                dictionary
                    .generate_answer(None)
                    .expect("Failed to generate answer.")
            }
        };
        let max_attempts = max_attempts.unwrap_or(match difficulty {
            GameDifficulty::Easy => 10,
            GameDifficulty::Medium => 6,
            GameDifficulty::Hard => 5,
        });
        return Game {
            state: GameState::Innit,
            word_length: answer.len(),
            max_attempts: max_attempts,
            guess_count: 0,
            answer: answer,
            dictionary: dictionary,
        };
    }
    pub fn guess(&mut self, input: &str) -> Result<crate::types::GuessResult, String> {
        if self.state != GameState::InProgress {
            return Err("Game is not in progress.".to_string());
        }
        let input = input.to_lowercase();
        if !input.chars().all(|c| c.is_ascii_lowercase()) {
            return Err("Word can only contain A-Z.".to_string());
        }
        if input.len() != self.word_length {
            return Err(format!("Input length must be {}.", self.word_length));
        }
        if !self.dictionary.is_valid_word(&input) {
            return Err("Word not found.".to_string());
        }
        self.guess_count += 1;
        match crate::game::judge::judge(&input, &self.answer) {
            Some(result) => {
                if result.iter().all(|state| *state == LetterState::Matched) {
                    self.state = GameState::Over(GameResult::Won);
                } else if self.guess_count >= self.max_attempts {
                    self.state = GameState::Over(GameResult::Lost);
                }
                return Ok(result);
            }
            None => return Err("Failed to judge the guess".to_string()),
        }
    }
    pub fn get_state(&self) -> GameState {
        self.state
    }
    pub fn get_answer(&self) -> &str {
        match self.state {
            GameState::Over(_) => &self.answer,
            _ => {
                panic!("Answer can only be retrieved when the game is over.");
            }
        }
    }
    pub fn start(&mut self) {
        self.state = GameState::InProgress;
    }
    pub fn get_attempts(&self) -> u8 {
        self.guess_count
    }
    pub fn get_max_attempts(&self) -> u8 {
        self.max_attempts
    }
    pub fn get_word_length(&self) -> usize {
        self.word_length
    }
    pub fn give_up(&mut self) {
        self.state = GameState::Over(GameResult::Lost);
    }
}
