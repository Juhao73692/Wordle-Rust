use core::fmt;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LetterState {
    Matched,
    Present,
    Absent,
}

pub type GuessResult = Vec<LetterState>;
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum GameResult {
    Won,
    Lost,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum GameState {
    Innit,
    InProgress,
    Over(GameResult),
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum GameDifficulty {
    Easy,
    Medium,
    Hard,
}

pub enum UiType {
    Console,
    Gui,
}

#[derive(Debug, Clone, Copy)]
pub enum ConsoleBackgroundColour {
    Green,
    Yellow,
    Gray,
    Red,
    Blue,
    Purple,
    Cyan,
    White,
    Reset,
}
impl ConsoleBackgroundColour {
    fn as_ansi(self) -> &'static str {
        match self {
            Self::Green => "\x1b[48;2;106;170;100m",
            Self::Yellow => "\x1b[48;2;201;180;88m",
            Self::Gray => "\x1b[48;2;120;124;126m",
            Self::Red => "\x1b[48;2;200;80;80m",
            Self::Blue => "\x1b[48;2;80;120;200m",
            Self::Purple => "\x1b[48;2;160;100;200m",
            Self::Cyan => "\x1b[48;2;80;180;180m",
            Self::White => "\x1b[48;2;220;220;220m",
            Self::Reset => "\x1b[0m",
        }
    }
    pub fn colour_text(self, text: &str) -> String {
        format!(
            "{}{}{}",
            self.as_ansi(),
            text,
            ConsoleBackgroundColour::Reset.as_ansi()
        )
    }
}

impl fmt::Display for ConsoleBackgroundColour {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_ansi())
    }
}

pub type ColourConfig = [ConsoleBackgroundColour; 3];
impl FromStr for ConsoleBackgroundColour {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ConsoleBackgroundColour::*;
        match s.to_ascii_lowercase().as_str() {
            "green" => Ok(Green),
            "yellow" => Ok(Yellow),
            "gray" | "grey" => Ok(Gray),
            "red" => Ok(Red),
            "blue" => Ok(Blue),
            "purple" => Ok(Purple),
            "cyan" => Ok(Cyan),
            "white" => Ok(White),
            _ => Err(format!("Unknown background colour: {}", s)),
        }
    }
}

#[derive(Debug)]
pub enum ConfigError {
    Io(std::io::Error),
    Toml(toml::de::Error),
    Missing(&'static str),
    InvalidColor(String),
}
