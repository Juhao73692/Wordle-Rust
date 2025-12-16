use crate::types::GameDifficulty;
use rand::Rng;
use serde_json::Value;
use std::collections::HashSet;
use std::fs;

pub struct Word {
    pub answer: Vec<Vec<String>>,
    pub dictionary: HashSet<String>,
}
impl Word {
    pub fn new(difficulty: &GameDifficulty) -> Self {
        let answer_path = match difficulty { // Make answers for different difficulties in the future
            GameDifficulty::Easy => "assets/word/answer.json",
            GameDifficulty::Medium => "assets/word/answer.json",
            GameDifficulty::Hard => "assets/word/answer.json",
        };
        let dictionary_path = "assets/word/dictionary.json";
        let answer: Vec<Vec<String>> =
            load_answer(answer_path).expect(format!("Failed to load {}", answer_path).as_str());
        let dictionary: HashSet<String> =
            load_dictionary(dictionary_path).expect(format!("Failed to load {}", dictionary_path).as_str());
        return Word { answer, dictionary };
    }
    pub fn is_valid_word(&self, word: &str) -> bool {
        self.dictionary.contains(word)
    }
    pub fn generate_answer(&self, word_length: Option<usize>) -> Result<String, &str> {
        let word_length = match word_length {
            Some(len) => len,
            None => {
                let mut rng = rand::rng();
                let x = rng.random::<f32>();
                if x < 0.2 {
                    4
                } else if x < 0.6 {
                    5
                } else if x < 0.8 {
                    6
                } else if x < 0.9 {
                    7
                } else if x < 0.95 {
                    8
                } else if x < 0.98 {
                    9
                } else if x < 0.99 {
                    10
                } else {
                    rng.random_range(11..=15)
                }
            }
        };
        if word_length >= self.answer.len() {
            return Err("Invalid word length.");
        }
        println!("Generating word of length: {}", word_length);
        let words = &self.answer[word_length];
        if words.is_empty() {
            return Err("No words available for the given length.");
        }
        let mut rng = rand::rng();
        let index = rng.random_range(0..words.len());
        return Ok(words[index].clone());
    }
}
fn load_dictionary(path: &str) -> Result<HashSet<String>, Box<dyn std::error::Error>> {
    let json_str = fs::read_to_string(path)?;
    let v: Value = serde_json::from_str(&json_str)?;

    let mut set = HashSet::new();

    if let Value::Object(map) = v {
        for (_len, words) in map {
            if let Value::Array(arr) = words {
                for word in arr {
                    if let Value::String(s) = word {
                        set.insert(s);
                    }
                }
            }
        }
    }

    Ok(set)
}

fn load_answer(path: &str) -> Result<Vec<Vec<String>>, Box<dyn std::error::Error>> {
    let json_str = fs::read_to_string(path)?;
    let v: serde_json::Value = serde_json::from_str(&json_str)?;

    let map = match v {
        serde_json::Value::Object(m) => m,
        _ => return Ok(Vec::new()),
    };
    let max_key = map
        .keys()
        .filter_map(|k| k.parse::<usize>().ok())
        .max()
        .unwrap_or(0);
    let mut lists = vec![Vec::new(); max_key + 1];
    for (key, value) in map {
        let idx = key.parse::<usize>()?;

        if let serde_json::Value::Array(arr) = value {
            let mut vec = Vec::new();
            for word in arr {
                if let serde_json::Value::String(s) = word {
                    vec.push(s);
                }
            }
            lists[idx] = vec;
        }
    }

    Ok(lists)
}
