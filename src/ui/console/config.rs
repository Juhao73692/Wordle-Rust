use ratatui::style::{Color, Style};
use std::str::FromStr;
use core::fmt;

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
    pub fn to_style(self) -> Style {
        match self {
            Self::Green  => Style::default().bg(Color::Rgb(106, 170, 100)),
            Self::Yellow => Style::default().bg(Color::Rgb(201, 180, 88)),
            Self::Gray   => Style::default().bg(Color::Rgb(120, 124, 126)),
            Self::Red    => Style::default().bg(Color::Rgb(200, 80, 80)),
            Self::Blue   => Style::default().bg(Color::Rgb(80, 120, 200)),
            Self::Purple => Style::default().bg(Color::Rgb(160, 100, 200)),
            Self::Cyan   => Style::default().bg(Color::Rgb(80, 180, 180)),
            Self::White  => Style::default().bg(Color::Rgb(220, 220, 220)),
            Self::Reset  => Style::default(),
        }
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

pub fn load_color_config() -> ColourConfig {
    const DEFAULT: ColourConfig = [
        ConsoleBackgroundColour::Green,
        ConsoleBackgroundColour::Yellow,
        ConsoleBackgroundColour::Gray,
    ];

    let content = match std::fs::read_to_string("assets/console/config.toml") {
        Ok(c) => c,
        Err(_) => return DEFAULT,
    };

    let value: toml::Value = match toml::from_str(&content) {
        Ok(v) => v,
        Err(_) => return DEFAULT,
    };

    let bg = match value
        .get("console")
        .and_then(|v| v.get("background"))
    {
        Some(v) => v,
        None => return DEFAULT,
    };

    let matched = match bg.get("matched").and_then(|v| v.as_str()) {
        Some(s) => match s.parse() {
            Ok(c) => c,
            Err(_) => return DEFAULT,
        },
        None => return DEFAULT,
    };

    let present = match bg.get("present").and_then(|v| v.as_str()) {
        Some(s) => match s.parse() {
            Ok(c) => c,
            Err(_) => return DEFAULT,
        },
        None => return DEFAULT,
    };

    let absent = match bg.get("absent").and_then(|v| v.as_str()) {
        Some(s) => match s.parse() {
            Ok(c) => c,
            Err(_) => return DEFAULT,
        },
        None => return DEFAULT,
    };

    [matched, present, absent]
}