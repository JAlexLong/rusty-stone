use std::fmt;

pub const BOARD_LETTERS: &str = "ABCDEFGHJKLMNOPQRST";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Stone {
    Black,
    White,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Point {
    pub row: usize,
    pub cell: usize,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        // return a string representation of the point

        write!(f, "({}, {})", self.row, self.cell)
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

    pub fn place_stone(&mut self, stone: Stone, point: Point) {
       // place the stone at the given coordinate
        self.state[point.row][point.cell] = Some(stone);
    }

    pub fn remove_stone(&mut self, point: Point) {
        // remove the stone at the given coordinate
        self.state[point.cell][point.cell] = None;
    }
}

impl<const N: usize> fmt::Display for Board<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        for row in self.state {
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

/*
████████╗███████╗███████╗████████╗    ███████╗ ██████╗ ███╗   ██╗███████╗
╚══██╔══╝██╔════╝██╔════╝╚══██╔══╝    ╚══███╔╝██╔═══██╗████╗  ██║██╔════╝
   ██║   █████╗  ███████╗   ██║         ███╔╝ ██║   ██║██╔██╗ ██║█████╗  
   ██║   ██╔══╝  ╚════██║   ██║        ███╔╝  ██║   ██║██║╚██╗██║██╔══╝  
   ██║   ███████╗███████║   ██║       ███████╗╚██████╔╝██║ ╚████║███████╗
   ╚═╝   ╚══════╝╚══════╝   ╚═╝       ╚══════╝ ╚═════╝ ╚═╝  ╚═══╝╚══════╝
                                                                         
*/

#[test]
fn test_new_board_is_empty() {
    let new_board: Board<19> = Board::new();

    for row in &new_board.state {
        for cell in row {
            match cell {
                Some(_) => panic!("Board should be empty"),
                _ => (),
            }
        }
    }
}

#[test]
fn test_place_stone() {
    let mut board: Board<19> = Board::new();
    let (row, cell) = (0, 0);
    let point = Point {row, cell};
    let stone = Stone::Black;

    board.place_stone(stone, point);

    assert_eq!(board.state[row][cell], Some(stone));
}

#[test]
fn test_remove_stone() {
    let mut board: Board<19> = Board::new();
    let (row, cell) = (0, 0);
    let point = Point {row, cell};
    let stone = Stone::Black;

    board.place_stone(stone, point.clone());
    board.remove_stone(point);

    assert_eq!(board.state[row][cell], None);
}