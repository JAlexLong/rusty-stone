use rusty_stone::board::Board;

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
