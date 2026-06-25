mod board;
mod moves;
mod bot;

use crate::board::Color;

fn square_to_algebraic(sq: usize) -> String {
    let col = (sq % 8) as u8;
    let row = (sq / 8) as u8;
    let col_char = (b'a' + col) as char;
    let row_char = (b'1' + row) as char;
    format!("{}{}", col_char, row_char)
}

fn other_color(color: &Color) -> Color {
    match color {
        Color::White => Color::Black,
        Color::Black => Color::White,
    }
}

fn main() {
    let mut board = board::new_board();
    board::print_board(&board);

    let search_depth = 3;
 
    let max_moves = 100;

    let mut turn = Color::White;

    for move_number in 1..=max_moves {
        let best_move = bot::find_best_move(&mut board, turn.clone(), search_depth);

        let (from, to) = match best_move {
            Some(m) => m,
            None => {
                println!("No legal moves left. Game over.");
                break;
            }
        };

        let move_str = format!("{}{}", square_to_algebraic(from), square_to_algebraic(to));
        moves::make_move(&mut board, &move_str);

        println!("Move {}: {:?} plays {}", move_number, turn, move_str);
        board::print_board(&board);

        turn = other_color(&turn);
    }
}

