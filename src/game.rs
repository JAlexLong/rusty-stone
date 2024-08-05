use std::fmt::{Display, Formatter};

use crate::board::{Board, Stone};

pub enum Handicap {
    Fixed,
    Free,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Player {
    Black,
    White,
}

impl Player {
    pub fn opponent(&self) -> Self {
        match *self {
            Player::White => Player::Black,
            Player::Black => Player::White,
        }
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let color = match *self {
            Player::Black => "black",
            Player::White => "white",
        };
        write!(f, "{color}")
    }
}

#[derive(Debug)]
pub struct Game {
    board: Board,
    turn: Stone,
    komi: f64,
}