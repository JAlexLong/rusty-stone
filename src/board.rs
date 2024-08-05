use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Stone {
    Black,
    White,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Coord {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Point {
    pub coord: Coord,
    pub state: Option<Stone>,
}

impl Point {
    pub fn is_empty(&self) -> bool {
        self.state.is_none()
    }
}

#[derive(Debug, Clone)]
pub struct Board<const N: usize = 19> {
    pub state: [[Option<Stone>; N]; N],
}

impl<const N: usize> Board<N> {
    pub fn new() -> Self {
        // return a new empty Board instance of the desired size
        let state: [[Option<Stone>; N]; N] = [[None; N]; N];
        Board {state}
    }
}

impl<const N: usize> fmt::Display for Board<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        for row in &self.state {
            for cell in row {
                match cell {
                    Some(Stone::Black) => write!(f, "X")?,
                    Some(Stone::White) => write!(f, "O")?,
                    None => write!(f, ".")?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
