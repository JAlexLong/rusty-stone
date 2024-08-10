use std::fmt;

use crate::board::{Board, Stone};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Player {
    name: &'static str,
    color: Stone,
}

impl Player {
    pub fn opponent(&self) -> Stone {
        match self.color {
            Stone::White => Stone::Black,
            Stone::Black => Stone::White,
        }
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let color: &str = match self.color {
            Stone::Black => "black",
            Stone::White => "white",
        };
        write!(f, "{color}")
    }
}

#[derive(Debug)]
pub struct Game {
    board: Board,
    turn: Stone,
    komi: f64,
    history: Vec<Board>,
}