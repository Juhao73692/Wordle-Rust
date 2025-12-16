use wordle::{types::GameDifficulty, ui::console::app::App as ConsoleGame};
fn main() {
    let mut game = ConsoleGame::new(&GameDifficulty::Easy);
    game.start();
}
