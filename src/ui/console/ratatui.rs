use std::io::stdout;
use std::time::Duration;

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
    },
};

use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};

use crossterm::event::KeyModifiers;

use crate::{
    game::game::Game,
    types::*,
    ui::console::config::*,
};

#[derive(Clone, Copy)]
enum Cell {
    Empty,
    Input(char),
    Result(char, LetterState),
}

pub struct App {
    game: Game,
    colour_config: ColourConfig,

    board: Vec<Vec<Cell>>,
    input: String,
    last_guess: String,

    show_warning: bool,
    warning_message: Option<String>,
    warning_ticks: u8,

    game_over_message: Option<String>,
    should_quit: bool,
}

impl App {
    pub fn new(difficulty: &GameDifficulty) -> Self {
        let colour_config = super::config::load_color_config();
        let game = Game::new(None, Some(*difficulty), None);

        let n = game.get_word_length();
        let k = game.get_max_attempts();

        Self {
            game,
            colour_config,
            board: vec![vec![Cell::Empty; n]; k as usize],
            input: String::new(),
            last_guess: String::new(),
            warning_message: None,
            show_warning: false,
            warning_ticks: 0,
            game_over_message: None,
            should_quit: false,
        }
    }

    /* ---------- terminal lifecycle ---------- */

    fn enter_terminal() {
        enable_raw_mode().unwrap();
        execute!(stdout(), EnterAlternateScreen).unwrap();
    }

    fn leave_terminal() {
        disable_raw_mode().unwrap();
        execute!(stdout(), LeaveAlternateScreen).unwrap();
    }

    /* ---------- main loop ---------- */

    pub fn start(&mut self) {
        self.game.start();
        Self::enter_terminal();

        let mut terminal =
            Terminal::new(CrosstermBackend::new(stdout())).unwrap();

        loop {
            if self.should_quit {
                break;
            }
            terminal.draw(|f| self.draw(f)).unwrap();

            if self.game_over_message.is_none() {
                if let GameState::Over(result) = self.game.get_state() {
                    self.game_over_message = Some(match result {
                        GameResult::Won => String::from("Congratulations!"),
                        GameResult::Lost => format!("Game over! The correct answer was: {}",
                            self.game.get_answer()),
                    });
                }
            }

            if event::poll(Duration::from_millis(120)).unwrap() {
                if let Event::Key(key) = event::read().unwrap() {
                    self.handle_key(key.code, key.modifiers);
                }
            }

            self.sync_input_to_board();
            self.tick_warning();
        }

        Self::leave_terminal();
        self.print_result();
    }

    /* ---------- input ---------- */

    fn handle_key(&mut self, code: KeyCode, modifiers: KeyModifiers) {
        if modifiers.contains(KeyModifiers::CONTROL) {
            if let KeyCode::Char('c') = code {
                self.should_quit = true;
                return;
            }
        }
        if code == KeyCode::Up {
            if self.last_guess != String::from(""){
                self.input = self.last_guess.clone();
            }
        }
        if self.game_over_message.is_some() {
            return;
        }

        let n = self.game.get_word_length();

        match code {
            KeyCode::Char(c)
                if c.is_ascii_alphabetic() && self.input.len() < n =>
            {
                self.input.push(c.to_ascii_lowercase());
            }
            KeyCode::Backspace => {
                self.input.pop();
            }
            KeyCode::Enter => {
                self.submit();
            }
            _ => {}
        }
    }

    fn submit(&mut self) {
        let row = self.game.get_attempts();

        match self.game.guess(&self.input) {
            Ok(result) => {
                for (col, state) in result.iter().enumerate() {
                    let ch = self.input.chars().nth(col).unwrap();
                    self.board[row as usize][col] = Cell::Result(ch, *state);
                }
                self.last_guess = self.input.clone();
                self.input.clear();
            }
            Err(err) => {
                self.warning_message = Some(String::from(format!("Error: {}", err)));
                self.flash_warning()
            },
        }
    }

    fn sync_input_to_board(&mut self) {
        let row = self.game.get_attempts();
        let n = self.game.get_word_length();

        if row >= self.board.len() as u8 {
            return;
        }

        for col in 0..n {
            self.board[row as usize][col] = match self.input.chars().nth(col) {
                Some(c) => Cell::Input(c),
                None => Cell::Empty,
            };
        }
    }

    /* ---------- warning ---------- */

    fn flash_warning(&mut self) {
        self.show_warning = true;
        self.warning_ticks = 6;
    }

    fn tick_warning(&mut self) {
        if self.warning_ticks > 0 {
            self.warning_ticks -= 1;
            if self.warning_ticks == 0 {
                self.show_warning = false;
            }
        }
    }

    /* ---------- drawing ---------- */

    fn draw(&self, f: &mut Frame) {
        let k = self.game.get_max_attempts();

        let layout = Layout::vertical([
            Constraint::Length(k as u16 + 2),
            Constraint::Length(1),
        ])
        .split(f.area());

        self.draw_board(f, layout[0]);
        self.draw_status(f, layout[1]);
    }

    fn draw_board(&self, f: &mut Frame, area: Rect) {
        let mut lines = Vec::new();

        for row in &self.board {
            let mut spans = Vec::new();
            for cell in row {
                let (ch, style) = self.render_cell(cell);
                spans.push(Span::styled(format!("[{}]", ch), style));
            }
            spans.push(Span::styled("[↵]", Style::default().fg(Color::DarkGray)));
            lines.push(Line::from(spans));
        }

        let p = Paragraph::new(lines)
            .block(Block::default().borders(Borders::ALL).title("Wordle"));

        f.render_widget(p, area);

        // cursor
        let row = self.game.get_attempts();
        let x = area.x + 1 + (self.input.len() as u16) * 3 + 1;
        let y = area.y + 1 + row as u16;
        f.set_cursor_position(Position { x: x, y: y });
    }

    fn render_cell(&self, cell: &Cell) -> (char, Style) {
        match cell {
            Cell::Empty => (' ', Style::default().fg(Color::DarkGray)),
            Cell::Input(c) => (*c, Style::default().fg(Color::White)),
            Cell::Result(c, state) => {
                let style = match state {
                    LetterState::Matched =>
                        self.colour_config[0].to_style().fg(Color::Black),
                    LetterState::Present =>
                        self.colour_config[1].to_style().fg(Color::Black),
                    LetterState::Absent =>
                        self.colour_config[2].to_style().fg(Color::Black),
                };
                (*c, style)
            }
        }
    }
    fn draw_status(&self, f: &mut Frame, area: Rect) {
        let (text, style) = if let Some(msg) = self.game_over_message.clone() {
            (msg, Style::default().fg(Color::Green))
        } else if self.show_warning {
            (self.warning_message.clone().unwrap(), Style::default().fg(Color::Red))
        } else {
            (String::from(""), Style::default())
        };

        let p = Paragraph::new(text);
        f.render_widget(p.style(style), area);
    }

    /* ---------- end ---------- */

    fn print_result(&self) {
        match self.game.get_state() {
            GameState::Over(GameResult::Won) => {
                println!("Congratulations! You won!");
            }
            GameState::Over(GameResult::Lost) => {
                println!(
                    "Game over! The correct answer was: {}",
                    self.game.get_answer()
                );
            }
            _ => {}
        }
    }
}