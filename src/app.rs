use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::layout::{Alignment, Rect};
use ratatui::style::Stylize;
use ratatui::widgets::{block::Title, Block, BorderType, Borders, Paragraph, Widget};
use ratatui::Frame;

use std::error;

use crate::game::Game;

pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

pub struct App {
    pub running: bool,
    size: Option<Rect>,
    game: Option<Game>,
}

impl App {
    pub fn new() -> Self {
        Self {
            running: true,
            size: None,
            game: None,
        }
    }

    pub fn tick(&mut self) {
        if self.game.is_some() && !self.game.as_ref().unwrap().over() {
            self.game.as_mut().unwrap().tick();
        }
    }

    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn handle_resize(&mut self) -> AppResult<()> {
        self.size = None;
        self.game = None;
        Ok(())
    }

    pub fn handle_input(&mut self, key_event: KeyEvent) -> AppResult<()> {
        //If the terminal size is not set yet, return early and skip first input
        if self.size.is_none() {
            return Ok(());
        }

        match key_event.code {
            // Exit application on `ESC` or `q`
            KeyCode::Esc | KeyCode::Char('q') => {
                self.quit();
            }
            // Exit application on `Ctrl-C`
            KeyCode::Char('c') | KeyCode::Char('C') => {
                if key_event.modifiers == KeyModifiers::CONTROL {
                    self.quit();
                }
            }

            KeyCode::Char(' ') => {
                if self.game.is_none() || self.game.as_ref().unwrap().over() {
                    self.game = Some(Game::new(self.size.unwrap()));
                }
            }

            KeyCode::Right | KeyCode::Left | KeyCode::Up | KeyCode::Down => {
                if let Some(game) = &mut self.game {
                    game.handle_input(key_event.code);
                }
            }

            _ => {}
        }
        Ok(())
    }

    pub fn render(&mut self, frame: &mut Frame) {
        if self.size.is_none() {
            self.size = Some(frame.size());
        }

        if self.game.is_none() || self.game.as_ref().unwrap().over() {
            Paragraph::new(
                "\nPress SPACE to start the game\n\nUse arrow keys to move\nPress 'q' to quit",
            )
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .title(Title::from("Snek").alignment(Alignment::Center)),
            )
            .alignment(Alignment::Center)
            .on_dark_gray()
            .render(frame.size(), frame.buffer_mut());
        }

        if let Some(game) = &mut self.game {
            game.render(frame);
        }
    }
}
