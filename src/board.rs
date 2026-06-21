//type Board = Vec<Option<Piece>>;
use std::fmt;

#[derive(Clone)]
#[derive(PartialEq)]
pub enum Color {White, Black}

#[derive(Clone)]
pub enum Kind {King, Queen, Bishop, Knight, Rook, Pawn}

#[derive(Clone)]
pub struct Piece {
    pub color: Color,
    pub kind: Kind,
}
// handle printing for the peices
impl fmt::Display for Piece {
    fn fmt(&self, f :&mut fmt::Formatter)-> fmt::Result {
        let symbol = match (&self.color, &self.kind) {
            (Color::Black, Kind::King)   => "♔",
            (Color::Black, Kind::Queen)  => "♕",
            (Color::Black, Kind::Rook)   => "♖",
            (Color::Black, Kind::Bishop) => "♗",
            (Color::Black, Kind::Knight) => "♘",
            (Color::Black, Kind::Pawn)   => "♙",
            (Color::White, Kind::King)   => "♚",
            (Color::White, Kind::Queen)  => "♛",
            (Color::White, Kind::Rook)   => "♜",
            (Color::White, Kind::Bishop) => "♝",
            (Color::White, Kind::Knight) => "♞",
            (Color::White, Kind::Pawn)   => "♟",
        };
        write!(f, "{}", symbol)
    }
}

//pub type Board = Vec<Option<Piece>>;

pub struct Board {
    white_pawns: u64,
    white_knights: u64,
    white_bishiops: u64,
    white_rooks: u64,
    white_queens: u64,
    white_king: u64,
    black_pawns: u64,
    black_knights: u64,
    black_bishops: u64,
    black_rooks: u64,
    black_queens: u64,
    black_king: u64,
}

impl Board {
    fn set_piece(&mut self, piece: Piece, square: usize){
        let symbol=match(piece.color, piece.kind) {
            (Color:: White, Kind::Pawn) => self.white_pawns |= 1u64 << square,
            (Color:: White, Kind::Bishop) => self.white_bishiops |= 1u64 << square,
            (Color:: White, Kind::Knight) => self.white_knights |= 1u64 << square,
            (Color:: White, Kind::Rook) => self.white_rooks |= 1u64 << square,
            (Color:: White, Kind::Queen) => self.white_queens |= 1u64 << square,
            (Color:: White, Kind::King) => self.white_king |= 1u64 << square,
            (Color:: Black, Kind::Pawn) => self.black_pawns |= 1u64 << square,
            (Color:: Black, Kind::Bishop) => self.black_bishiops |= 1u64 << square,
            (Color:: Black, Kind::Knight) => self.black_knights |= 1u64 << square,
            (Color:: Black, Kind::Rook) => self.black_rooks |= 1u64 << square,
            (Color:: Black, Kind::Queen) => self.black_queens |= 1u64 << square,
            (Color:: Black, Kind::King) => self.black_king |= 1u64 << square,
        }
    }
    fn clear_piece(&mut self, piece: Piece, square: usize){
        let symbol=match(piece.color, piece.kind) {
            (Color:: White, Kind::Pawn) => self.white_pawns &= !(1u64 << square),
            (Color:: White, Kind::Bishop) => self.white_bishiops &= !(1u64 << square),
            (Color:: White, Kind::Knight) => self.white_knights &= !(1u64 << square),
            (Color:: White, Kind::Rook) => self.white_rooks &= !(1u64 << square),
            (Color:: White, Kind::Queen) => self.white_queens &= !(1u64 << square),
            (Color:: White, Kind::King) => self.white_king &= !(1u64 << square),
            (Color:: Black, Kind::Pawn) => self.black_pawns &= !(1u64 << square),
            (Color:: Black, Kind::Bishop) => self.black_bishiops &= !(1u64 << square),
            (Color:: Black, Kind::Knight) => self.black_knights &= !(1u64 << square),
            (Color:: Black, Kind::Rook) => self.black_rooks &= !(1u64 << square),
            (Color:: Black, Kind::Queen) => self.black_queens &= !(1u64 << square),
            (Color:: Black, Kind::King) => self.black_king &= !(1u64 << square),
        }
    }
    //let is_white_pawn_here = self.white_pawns & (1u64 << square) != 0;
    fn piece_at_square(&self, square: usize) -> Option<Piece>{
        if (self.white_pawns & (1u64 << square !=0)) {
            Some(Piece { color: Color::White, kind: Kind::Pawn})
        } else if (self.white_knights & (1u64 << square) !=0) {
            Some(Piece { color: Color::White, kind: Kind::Knight})
        } else if (self.white_bishiops & (1u64 << square) !=0) {
            Some(Piece { color: Color::White, kind: Kind::Bishop})
        } else if (self.white_rooks & (1u64 << square) !=0) {
            Some(Piece { color: Color::White, kind: Kind::Rook})
        } else if (self.white_queens & (1u64 << square) !=0) {
            Some(Piece { color: Color::White, kind: Kind::Queen})
        } else if (self.white_king & (1u64 << square) !=0) {
            Some(Piece { color: Color::White, kind: Kind::King})
        } if (self.black_pawns & (1u64 << square !=0)) {
            Some(Piece { color: Color::Black, kind: Kind::Pawn})
        } else if (self.black_knights & (1u64 << square) !=0) {
            Some(Piece { color: Color::Black, kind: Kind::Knight})
        } else if (self.black_bishiops & (1u64 << square) !=0) {
            Some(Piece { color: Color::Black, kind: Kind::Bishop})
        } else if (self.black_rooks & (1u64 << square) !=0) {
            Some(Piece { color: Color::Black, kind: Kind::Rook})
        } else if (self.black_queens & (1u64 << square) !=0) {
            Some(Piece { color: Color::Black, kind: Kind::Queen})
        } else if (self.black_king & (1u64 << square) !=0) {
            Some(Piece { color: Color::Black, kind: Kind::King})
        }
    }
}

pub fn new_board()->Board{
    let mut board:Board  = vec![None; 64];
    let back_rank= [Kind::Rook, Kind::Knight, Kind::Bishop, Kind::Queen, Kind::King, Kind::Bishop, Kind:: Knight, Kind::Rook];
    
    for (i, kind) in back_rank.into_iter().enumerate() {
        board[i]      = Some(Piece { color: Color::White, kind: kind.clone() });
        board[8 + i]  = Some(Piece { color: Color::White, kind: Kind::Pawn });
        board[48 + i] = Some(Piece { color: Color::Black, kind: Kind::Pawn });
        board[56 + i] = Some(Piece { color: Color::Black, kind });
    }

    board
}

pub fn print_board(board: &Board){
    println!(" |{:^3}|{:^3}|{:^3}|{:^3}|{:^3}|{:^3}|{:^3}|{:^3}|", "a", "b", "c", "d", "e", "f", "g", "h");
    println!("{}","-".repeat(34));

    for i in (0..8).rev(){
        print!("{}|",i+1);
        for j in 0..8 {
            let cell = match &board[i * 8 + j] {
                Some(piece) => format!("{}", piece),
                None        => " ".to_string(),
            };
            print!("{:^3}|", cell);
        }
        println!{};
        println!("{}","-".repeat(34));
    }
    println!(" |{:^3}|{:^3}|{:^3}|{:^3}|{:^3}|{:^3}|{:^3}|{:^3}|", "a", "b", "c", "d", "e", "f", "g", "h");
    println!("{}","-".repeat(34));


}

