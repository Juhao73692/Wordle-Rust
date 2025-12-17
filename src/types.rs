
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

pub enum ConsoleUiType {
    Ratatui,
    Plain,
}

pub enum UiType {
    Console(ConsoleUiType),
    Gui,
}