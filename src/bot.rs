use crate::board::{Board, Color, Kind, Piece};
use crate::moves::get_legal_moves;

const PAWN_VALUE: i32 = 100;
const KNIGHT_VALUE: i32 = 320;
const BISHOP_VALUE: i32 = 330;
const ROOK_VALUE: i32 = 500;
const QUEEN_VALUE: i32 = 900;
const KING_VALUE: i32 = 20000;

const INFINITY: i32 = 1_000_000_000;

fn piece_value(kind: &Kind) -> i32 {
    match kind {
        Kind::Pawn => PAWN_VALUE,
        Kind::Knight => KNIGHT_VALUE,
        Kind::Bishop => BISHOP_VALUE,
        Kind::Rook => ROOK_VALUE,
        Kind::Queen => QUEEN_VALUE,
        Kind::King => KING_VALUE,
    }
}

fn evaluate(board: &Board) -> i32 {
    let mut score = 0;

    for square in 0..64 {
        if let Some(piece) = board.piece_at_square(square) {
            let value = piece_value(&piece.kind);

            if piece.color == Color::White {
                score += value;
            } else {
                score -= value;
            }
        }
    }

    score
}

fn other_color(color: &Color) -> Color {
    match color {
        Color::White => Color::Black,
        Color::Black => Color::White,
    }
}

fn do_move(board: &mut Board, from: usize, to: usize, piece: &Piece) -> Option<Piece> {
    let captured = board.piece_at_square(to);

    board.clear_piece(piece.clone(), from);

    if let Some(captured_piece) = captured.clone() {
        board.clear_piece(captured_piece, to);
    }

    board.set_piece(piece.clone(), to);

    captured
}

fn undo_move(board: &mut Board, from: usize, to: usize, piece: &Piece, captured: Option<Piece>) {
    board.clear_piece(piece.clone(), to);

    if let Some(captured_piece) = captured {
        board.set_piece(captured_piece, to);
    }

    board.set_piece(piece.clone(), from);
}

fn negamax(board: &mut Board, depth: u32, alpha: i32, beta: i32, color: Color) -> i32 {

    if depth == 0 {
        let eval = evaluate(board);

        return match color {
            Color::White => eval,
            Color::Black => -eval,
        };
    }

    let moves = get_legal_moves(board, color.clone());

    if moves.is_empty() {
        return -KING_VALUE;
    }

    let mut best_score = i32::MIN;
    let mut alpha = alpha;

    for (from, to, piece) in moves {

        let captured = do_move(board, from, to, &piece);

        let score = -negamax(board, depth - 1, -beta, -alpha, other_color(&color));

        undo_move(board, from, to, &piece, captured);

        if score > best_score {
            best_score = score;
        }

        if best_score > alpha {
            alpha = best_score;
        }

        if alpha >= beta {
            break;
        }
    }

    best_score
}

pub fn find_best_move(board: &mut Board, color: Color, depth: u32) -> Option<(usize, usize)> {
    let moves = get_legal_moves(board, color.clone());

    let mut best_move = None;
    let mut best_score = -INFINITY;

    let mut alpha = -INFINITY;
    let beta = INFINITY;

    for (from, to, piece) in moves {
        let captured = do_move(board, from, to, &piece);

        let score = -negamax(board, depth - 1, -beta, -alpha, other_color(&color));

        undo_move(board, from, to, &piece, captured);

        if score > best_score {
            best_score = score;
            best_move = Some((from, to));
        }

        if best_score > alpha {
            alpha = best_score;
        }
    }

    best_move
}
