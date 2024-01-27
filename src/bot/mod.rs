use core::cmp::{max, min};
use std::f32::INFINITY;

use chess_engine::*;

fn make_move(position: ChessBoard) -> ChessMove {

}

fn minimax(position: ChessBoard, depth: usize, maximizing_player_is_white: bool) -> f32 {
	if depth == 0 {
		// evaluate the position
	}

	if !(maximizing_player_is_white ^ position.turn) {
		let mut max_evaluation = -INFINITY;
		for chess_move in position.available_moves() {
			let evaluation = minimax(position.move_piece(&chess_move), depth - 1, maximizing_player_is_white);
			max_evaluation = max_evaluation.max(evaluation);
		}
		return max_evaluation
	} else {
		let mut min_evaluation = INFINITY;
		for chess_move in position.available_moves() {
			let evaluation = minimax(position.move_piece(&chess_move), depth - 1, maximizing_player_is_white);
			min_evaluation = min_evaluation(evaluation);
		}
		return min_evaluation
	}
}
