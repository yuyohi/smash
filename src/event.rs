use crossterm::event::{Event as TermEvent, KeyCode, KeyModifiers};
use crossterm::queue;
use crossterm::style::{Attribute, Print, SetAttribute};
use crossterm::terminal::{self, disable_raw_mode, enable_raw_mode};
use std::io::Write;
use std::time::Duration;
use tracing::debug;

#[derive(Clone, Debug)]
struct UserInput {
    input: String
}

#[derive(Clone, Debug)]
pub struct SmashState {
    columns: usize,
    lines: usize,
    prompt_len: usize,
    input: UserInput
}

impl UserInput {
    pub fn new() -> Self {
        Self {
            input: String::with_capacity(256),
        }
    }
}

impl SmashState {
    pub fn new() -> Self {
        Self {
            columns: 0,
            lines: 0,
            prompt_len: 0,
            input: UserInput::new(),
        }
    }

    pub fn render_prompt(&mut self) {
        let screen_size = terminal::size().unwrap()
    }

    pub fn run(&mut self) {
        todo!()
    }
}