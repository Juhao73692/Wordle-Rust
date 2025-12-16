use crate::types::*;

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