use crate::types::{GuessResult, LetterState};
use core::panic;

pub fn judge(guess: &str, answer: &str) -> Option<GuessResult> {
    if guess.len() != answer.len() {
        panic!("Guess and answer must be of the same length");
    }
    let mut result = vec![LetterState::Absent; guess.len()];
    let guess_chars: Vec<char> = guess.chars().collect();
    let mut answer_chars: Vec<char> = answer.chars().collect();
    for i in 0..guess_chars.len() {
        if guess_chars[i] == answer_chars[i] {
            result[i] = LetterState::Matched;
            answer_chars[i] = '_';
        }
    }
    for i in 0..guess_chars.len() {
        if result[i] == LetterState::Matched {
            continue;
        }
        if answer_chars.contains(&guess_chars[i]) {
            result[i] = LetterState::Present;
            let index = answer_chars
                .iter()
                .position(|&c| c == guess_chars[i])
                .unwrap();
            answer_chars[index] = '_';
        }
    }
    return Some(result);
}
