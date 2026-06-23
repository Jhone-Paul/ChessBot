mod board;
mod moves;

fn square_to_algebraic(sq: usize) -> String {
    let col = (sq % 8) as u8;
    let row = (sq / 8) as u8;
    let col_char = (b'a' + col) as char;
    let row_char = (b'1' + row) as char;
    format!("{}{}", col_char, row_char)
}

fn main() {
    let mut bbldrizzy = board::new_board();
    board::print_board(&bbldrizzy);

    let pos = moves::parse_square("e4");
    //print!("{}",pos);

    moves::make_move(&mut bbldrizzy, "e2e4");
    board::print_board(&bbldrizzy);
    
    let moves = moves::get_legal_moves(&bbldrizzy, board::Color::White);
    for (from, to) in &moves {
        println!("{}{}", square_to_algebraic(*from), square_to_algebraic(*to));
    }
}
