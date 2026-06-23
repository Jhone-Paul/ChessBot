mod board;
mod moves;

fn main() {
    let mut bbldrizzy = board::new_board();
    board::print_board(&bbldrizzy);

    let pos = moves::parse_square("e4");
    //print!("{}",pos);

    moves::make_move(&mut bbldrizzy, "e2e4");
    board::print_board(&bbldrizzy);
}
