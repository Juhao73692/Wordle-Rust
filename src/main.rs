use wordle::{types::GameDifficulty, ui::console::plain_console::App as ConsoleGame, ui::console::ratatui::App as RatatuiGame};
use wordle::types::{UiType, ConsoleUiType};
fn main() {
    let ui_type = UiType::Console(ConsoleUiType::Ratatui);
    match ui_type {
        UiType::Console(console_type) => match console_type {
            ConsoleUiType::Plain => {
                let mut game = ConsoleGame::new(&GameDifficulty::Easy);
                game.start();
            }
            ConsoleUiType::Ratatui => {
                let mut game = RatatuiGame::new(&GameDifficulty::Easy);
                game.start();
            }
        },
        UiType::Gui => {
            // GUI not implemented yet
        }
    }
}
