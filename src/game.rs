use crate::board::{Board, Stone};
use crate::rules::*;

pub struct Player {
    color: Stone,
    captures: usize,
    komi: Komi,
}

#[derive(Debug)]
pub struct Game {
    board: Board,
    size: usize,
    turn: Stone,
}