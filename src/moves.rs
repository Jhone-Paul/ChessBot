use crate::board::Board;
use crate::board::Color;
use crate::board::Kind;
use crate::board::Piece;

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

const ROOK_DIRS: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
const BISHOP_DIRS: [(i32, i32); 4] = [(1, 1), (1, -1), (-1, 1), (-1, -1)];
const QUEEN_DIRS: [(i32, i32); 8] = [
    (1, 0), (-1, 0), (0, 1), (0, -1),
    (1, 1), (1, -1), (-1, 1), (-1, -1),
];
const KNIGHT_DELTAS: [(i32, i32); 8] = [
    (1, 2), (2, 1), (2, -1), (1, -2),
    (-1, -2), (-2, -1), (-2, 1), (-1, 2),
];
const KING_DELTAS: [(i32, i32); 8] = [
    (1, 0), (-1, 0), (0, 1), (0, -1),
    (1, 1), (1, -1), (-1, 1), (-1, -1),
];

fn rank_file_to_square(rank: i32, file: i32) -> Option<usize> {
    if (0..8).contains(&rank) && (0..8).contains(&file) {
        Some((rank * 8 + file) as usize)
    } else {
        None
    }
}

fn square_to_rank_file(sq: usize) -> (i32, i32) {
    ((sq / 8) as i32, (sq % 8) as i32)
}
fn generate_pawn_moves(board: &Board, sq: usize, color: &Color, moves: &mut Vec<(usize, usize, Piece)>) {
    let (rank, file) = square_to_rank_file(sq);
    let (dir, start_rank) = match color {
        Color::White => (1, 1),
        Color::Black => (-1, 6),
    };
    let piece = Piece { color: color.clone(), kind: Kind::Pawn };

    if let Some(target) = rank_file_to_square(rank + dir, file) {
        if board.piece_at_square(target).is_none() {
            moves.push((sq, target, piece.clone()));

            if rank == start_rank {
                if let Some(target2) = rank_file_to_square(rank + 2 * dir, file) {
                    if board.piece_at_square(target2).is_none() {
                        moves.push((sq, target2, piece.clone()));
                    }
                }
            }
        }
    }

    for df in [-1, 1] {
        if let Some(target) = rank_file_to_square(rank + dir, file + df) {
            if let Some(p) = board.piece_at_square(target) {
                if p.color != *color {
                    moves.push((sq, target, piece.clone()));
                }
            }
        }
    }
}

fn generate_knight_moves(board: &Board, sq: usize, color: &Color, moves: &mut Vec<(usize, usize, Piece)>) {
    let (rank, file) = square_to_rank_file(sq);
    let piece = Piece { color: color.clone(), kind: Kind::Knight };
    for (dr, df) in KNIGHT_DELTAS {
        if let Some(target) = rank_file_to_square(rank + dr, file + df) {
            match board.piece_at_square(target) {
                Some(p) if p.color == *color => {}
                _ => moves.push((sq, target, piece.clone())),
            }
        }
    }
}

fn generate_king_moves(board: &Board, sq: usize, color: &Color, moves: &mut Vec<(usize, usize, Piece)>) {
    let (rank, file) = square_to_rank_file(sq);
    let piece = Piece { color: color.clone(), kind: Kind::King };
    for (dr, df) in KING_DELTAS {
        if let Some(target) = rank_file_to_square(rank + dr, file + df) {
            match board.piece_at_square(target) {
                Some(p) if p.color == *color => {}
                _ => moves.push((sq, target, piece.clone())),
            }
        }
    }
}

fn generate_sliding_moves(
    board: &Board,
    sq: usize,
    color: &Color,
    kind: Kind,
    dirs: &[(i32, i32)],
    moves: &mut Vec<(usize, usize, Piece)>,
) {
    let (rank, file) = square_to_rank_file(sq);
    let piece = Piece { color: color.clone(), kind };
    for (dr, df) in dirs {
        let mut r = rank + dr;
        let mut f = file + df;
        while let Some(target) = rank_file_to_square(r, f) {
            match board.piece_at_square(target) {
                None => moves.push((sq, target, piece.clone())),
                Some(p) => {
                    if p.color != *color {
                        moves.push((sq, target, piece.clone()));
                    }
                    break;
                }
            }
            r += dr;
            f += df;
        }
    }
}

pub fn get_legal_moves(board: &Board, color: Color) -> Vec<(usize, usize, Piece)> {
    let mut moves = Vec::new();

    for sq in 0..64 {
        if let Some(piece) = board.piece_at_square(sq) {
            if piece.color == color {
                match piece.kind {
                    Kind::Pawn => generate_pawn_moves(board, sq, &color, &mut moves),
                    Kind::Knight => generate_knight_moves(board, sq, &color, &mut moves),
                    Kind::Bishop => generate_sliding_moves(board, sq, &color, Kind::Bishop, &BISHOP_DIRS, &mut moves),
                    Kind::Rook => generate_sliding_moves(board, sq, &color, Kind::Rook, &ROOK_DIRS, &mut moves),
                    Kind::Queen => generate_sliding_moves(board, sq, &color, Kind::Queen, &QUEEN_DIRS, &mut moves),
                    Kind::King => generate_king_moves(board, sq, &color, &mut moves),
                }
            }
        }
    }

    moves
}

