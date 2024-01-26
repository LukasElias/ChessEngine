extern crate chess_engine;

use chess_engine::*;
mod bot;

fn main() {
    let board = ChessBoard::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string());
}