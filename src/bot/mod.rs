use std::f32::INFINITY;

use chess_engine::*;

fn make_move(position: ChessBoard) -> ChessMove {

}

fn minimax(position: ChessBoard, depth: usize, maximizing_player_is_white: bool) -> ChessMove {
	if depth == 0 {
		// evaluate the position
	}

	if !(maximizing_player_is_white ^ position.is_white) {
		let max_evaluation = -INFINITY;
		for move in position.available_moves() {
			
		}
	}
}