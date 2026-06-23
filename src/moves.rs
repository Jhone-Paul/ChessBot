use crate::board::Board;
use crate::board::Color;
use crate::board::Kind;
const DEBUG: bool= true;
//takes in a chess square (like e4) and returns the bitstring of that peice.
pub fn parse_square(s:&str) -> usize {

    let mut chars = s.chars();
    let col = chars.next().unwrap();
    let row = chars.next().unwrap();
    
    let col = (col as usize) - ('a' as usize);
    let row = (row as usize) - ('1' as usize);

    let pos = row * 8 + col as usize;
    pos
    // let mut board_pos = 0u64;
    // board_pos +=1;
    
    // board_pos.rotate_left(pos)
}

pub fn make_move(board: &mut Board, s: &str) {
    let from = parse_square(&s[0..2]);
    let to   = parse_square(&s[2..4]);
    
    let piece = Board::piece_at_square(&board, from);
    if DEBUG {println!{"{}",format!{"{}",piece.clone().unwrap()}}};

    Board::clear_piece(board, piece.clone().unwrap(), from);
    Board::set_piece(board, piece.clone().unwrap(), to);
}
