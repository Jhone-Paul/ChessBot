mod board;

fn main() {
    let bbldrizzy = board::new_board();
    board::print_board(&bbldrizzy);
}
