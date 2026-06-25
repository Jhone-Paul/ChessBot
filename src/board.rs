//type Board = Vec<Option<Piece>>;
use std::fmt;

#[derive(Clone)]
#[derive(PartialEq, Debug)]
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
    white_bishops: u64,
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
    pub fn set_piece(&mut self, piece: Piece, square: usize){
        match(piece.color, piece.kind) {
            (Color:: White, Kind::Pawn) => self.white_pawns |= 1u64 << square,
            (Color:: White, Kind::Bishop) => self.white_bishops |= 1u64 << square,
            (Color:: White, Kind::Knight) => self.white_knights |= 1u64 << square,
            (Color:: White, Kind::Rook) => self.white_rooks |= 1u64 << square,
            (Color:: White, Kind::Queen) => self.white_queens |= 1u64 << square,
            (Color:: White, Kind::King) => self.white_king |= 1u64 << square,
            (Color:: Black, Kind::Pawn) => self.black_pawns |= 1u64 << square,
            (Color:: Black, Kind::Bishop) => self.black_bishops |= 1u64 << square,
            (Color:: Black, Kind::Knight) => self.black_knights |= 1u64 << square,
            (Color:: Black, Kind::Rook) => self.black_rooks |= 1u64 << square,
            (Color:: Black, Kind::Queen) => self.black_queens |= 1u64 << square,
            (Color:: Black, Kind::King) => self.black_king |= 1u64 << square,
        }
    }
    pub fn clear_piece(&mut self, piece: Piece, square: usize){
        match(piece.color, piece.kind) {
            (Color:: White, Kind::Pawn) => self.white_pawns &= !(1u64 << square),
            (Color:: White, Kind::Bishop) => self.white_bishops &= !(1u64 << square),
            (Color:: White, Kind::Knight) => self.white_knights &= !(1u64 << square),
            (Color:: White, Kind::Rook) => self.white_rooks &= !(1u64 << square),
            (Color:: White, Kind::Queen) => self.white_queens &= !(1u64 << square),
            (Color:: White, Kind::King) => self.white_king &= !(1u64 << square),
            (Color:: Black, Kind::Pawn) => self.black_pawns &= !(1u64 << square),
            (Color:: Black, Kind::Bishop) => self.black_bishops &= !(1u64 << square),
            (Color:: Black, Kind::Knight) => self.black_knights &= !(1u64 << square),
            (Color:: Black, Kind::Rook) => self.black_rooks &= !(1u64 << square),
            (Color:: Black, Kind::Queen) => self.black_queens &= !(1u64 << square),
            (Color:: Black, Kind::King) => self.black_king &= !(1u64 << square),
        }
    }
    //let is_white_pawn_here = self.white_pawns & (1u64 << square) != 0;
    pub fn piece_at_square(&self, square: usize) -> Option<Piece>{
        if (self.white_pawns & (1u64 << square)) !=0 {
            Some(Piece { color: Color::White, kind: Kind::Pawn})
        } else if (self.white_knights & (1u64 << square)) !=0 {
            Some(Piece { color: Color::White, kind: Kind::Knight})
        } else if (self.white_bishops & (1u64 << square)) !=0 {
            Some(Piece { color: Color::White, kind: Kind::Bishop})
        } else if (self.white_rooks & (1u64 << square)) !=0 {
            Some(Piece { color: Color::White, kind: Kind::Rook})
        } else if (self.white_queens & (1u64 << square)) !=0 {
            Some(Piece { color: Color::White, kind: Kind::Queen})
        } else if (self.white_king & (1u64 << square)) !=0 {
            Some(Piece { color: Color::White, kind: Kind::King})
        } else if (self.black_pawns & (1u64 << square)) !=0 {
            Some(Piece { color: Color::Black, kind: Kind::Pawn})
        } else if (self.black_knights & (1u64 << square)) !=0 {
            Some(Piece { color: Color::Black, kind: Kind::Knight})
        } else if (self.black_bishops & (1u64 << square)) !=0 {
            Some(Piece { color: Color::Black, kind: Kind::Bishop})
        } else if (self.black_rooks & (1u64 << square)) !=0 {
            Some(Piece { color: Color::Black, kind: Kind::Rook})
        } else if (self.black_queens & (1u64 << square)) !=0 {
            Some(Piece { color: Color::Black, kind: Kind::Queen})
        } else if (self.black_king & (1u64 << square)) !=0 {
            Some(Piece { color: Color::Black, kind: Kind::King})
        } else {
            None
        }
    }
}

pub fn new_board() -> Board {
    let mut board = Board {
        white_pawns: 0,
        white_knights: 0,
        white_bishops: 0,
        white_rooks: 0,
        white_queens: 0,
        white_king: 0,

        black_pawns: 0,
        black_knights: 0,
        black_bishops: 0,
        black_rooks: 0,
        black_queens: 0,
        black_king: 0,
    };

    let back_rank = [
        Kind::Rook,
        Kind::Knight,
        Kind::Bishop,
        Kind::Queen,
        Kind::King,
        Kind::Bishop,
        Kind::Knight,
        Kind::Rook,
    ];

    for (i, kind) in back_rank.into_iter().enumerate() {
        board.set_piece(Piece { color: Color::White, kind: kind.clone() }, i);
        board.set_piece(Piece { color: Color::White, kind: Kind::Pawn }, 8 + i);

        board.set_piece(Piece { color: Color::Black, kind: Kind::Pawn }, 48 + i);
        board.set_piece(Piece { color: Color::Black, kind }, 56 + i);
    }

    board
}

pub fn print_board(board: &Board) {
    println!(" |{:^3}|{:^3}|{:^3}|{:^3}|{:^3}|{:^3}|{:^3}|{:^3}|",
             "a", "b", "c", "d", "e", "f", "g", "h");
    println!("{}", "-".repeat(34));

    for rank in (0..8).rev() {
        print!("{}|", rank + 1);

        for file in 0..8 {
            let square = rank * 8 + file;

            let cell = match board.piece_at_square(square) {
                Some(piece) => format!("{}", piece),
                None => " ".to_string(),
            };

            print!("{:^3}|", cell);
        }

        println!();
        println!("{}", "-".repeat(34));
    }

    println!(" |{:^3}|{:^3}|{:^3}|{:^3}|{:^3}|{:^3}|{:^3}|{:^3}|",
             "a", "b", "c", "d", "e", "f", "g", "h");
}

