use std::collections::VecDeque;

use crossterm::event::KeyCode;

use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Style},
    widgets::{block::Title, Block, BorderType, Widget},
    Frame,
};

pub struct Game {
    state: State,
    score: u16,
    head: Coordinates,
    tail: VecDeque<Coordinates>,
    food: Vec<Coordinates>,
    direction: Direction,
    size: Rect,
}

pub enum State {
    Running,
    GameOver,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Coordinates {
    x: u16,
    y: u16,
}

#[derive(Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Game {
    pub fn new(rect: Rect) -> Self {
        let middle = Coordinates {
            x: (rect.width - 2) / 2 + 1,
            y: (rect.height - 2) / 2 + 1,
        };

        Self {
            state: State::Running,
            score: 0,
            head: middle,
            tail: VecDeque::new(),
            food: Vec::new(),
            direction: Direction::Up,
            size: rect,
        }
    }

    pub fn over(&self) -> bool {
        matches!(self.state, State::GameOver)
    }

    pub fn handle_input(&mut self, key: KeyCode) {
        self.direction = match key {
            KeyCode::Up => Direction::Up,
            KeyCode::Down => Direction::Down,
            KeyCode::Left => Direction::Left,
            KeyCode::Right => Direction::Right,
            _ => self.direction,
        };
    }

    pub fn handle_movement(&mut self) {
        self.tail.push_front(self.head);

        match self.direction {
            Direction::Up => self.head.y -= 1,
            Direction::Down => self.head.y += 1,
            Direction::Left => self.head.x -= 1,
            Direction::Right => self.head.x += 1,
        };
    }

    pub fn handle_collision_tail(&mut self) {
        if self.tail.contains(&self.head) {
            self.state = State::GameOver;
        };
    }

    pub fn handle_collision_border(&mut self) {
        if self.head.x == 0
            || self.head.y == 0
            || self.head.x == self.size.width - 1
            || self.head.y == self.size.height - 1
        {
            self.state = State::GameOver;
        }
    }

    pub fn handle_collision_food(&mut self) {
        if self.food.contains(&self.head) {
            self.food.retain(|&food| food != self.head);
            self.score += 1;
        } else {
            self.tail.pop_back();
        }
    }

    pub fn spawn_food(&mut self) {
        while self.food.is_empty() {
            let maybe_new_food = Coordinates {
                x: rand::random::<u16>() % (self.size.width - 2) + 1,
                y: rand::random::<u16>() % (self.size.height - 2) + 1,
            };

            if !self.tail.contains(&maybe_new_food) && maybe_new_food != self.head {
                self.food.push(maybe_new_food);
            }
        }
    }

    pub fn tick(&mut self) {
        self.handle_movement();
        self.handle_collision_tail();
        self.handle_collision_border();
        self.handle_collision_food();
        self.spawn_food();
    }

    pub fn render(&mut self, frame: &mut Frame) {
        let size = frame.size();
        let buf = frame.buffer_mut();

        Block::bordered()
            .title(Title::from(format!("Score: {:?}", self.score)).alignment(Alignment::Left))
            .title(Title::from("Snek").alignment(Alignment::Center))
            .border_type(BorderType::Rounded)
            .render(size, buf);

        buf.set_string(
            self.head.x,
            self.head.y,
            "█",
            Style::default().fg(Color::LightGreen),
        );

        for snake_part in &self.tail {
            buf.set_string(
                snake_part.x,
                snake_part.y,
                "█",
                Style::default().fg(Color::Green),
            );
        }

        for food in &self.food {
            buf.set_string(food.x, food.y, "O", Style::default().fg(Color::Red));
        }
        // buf.set_string(17, 10, "O", Style::default().fg(Color::Green));
        // buf.set_string(35, 16, "F", Style::default().fg(Color::Red));
    }
}
