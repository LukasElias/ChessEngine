use {
    crate::bot::{
        GoOptions,
    },
    chess::{
        ChessMove,
        MoveGen,
        Board,
        Piece,
        Color,
        CastleRights,
        ALL_PIECES,
    },
    std::{
        sync::{
            atomic::{
                AtomicBool,
                Ordering,
            },
            mpsc,
            Arc,
        },
        io::{
            Write,
            stdout,
        },
        thread,
        f32::NEG_INFINITY,
    }
};

#[derive(Debug, Clone)]
pub struct SearchThread {
    pub search_state: Arc<SearchState>,
    pub sender: mpsc::Sender<GoOptions>,
}

#[derive(Debug, Default)]
pub struct SearchState {
    pub stop_flag: AtomicBool,
    pub busy_flag: AtomicBool,
}

impl Default for SearchThread {
    fn default() -> Self {
        let search_state = Arc::new(SearchState::default());

        let (sender, receiver) = mpsc::channel::<GoOptions>();
        let state_clone = Arc::clone(&search_state);

        thread::spawn(move || {
            let mut stdout = stdout();
            while let Ok(go_options) = receiver.recv() {
                if state_clone.stop_flag.load(Ordering::Relaxed) {
                    break
                }

                // now we're busy
                state_clone.busy_flag.store(true, Ordering::Relaxed);

                // TODO: Calculate the move here

                let calculated_move = search_moves(go_options, &state_clone);

                // Make a move

                writeln!(stdout, "bestmove {}", calculated_move).unwrap();
                stdout.flush().unwrap();

                state_clone.busy_flag.store(false, Ordering::Relaxed);
            }
        });

        Self {
            search_state: search_state,
            sender,
        }
    }
}

// This can block as much as it want's as long as it returns when it gets a stop flag or it meets one of the requirements from the GoOptions
fn search_moves(go_options: GoOptions, _state: &Arc<SearchState>) -> ChessMove {
    // The idea is that we in the future sets a time frame for the searching, so we use the
    // GoOptions to figure out how much time we should use searching. Then afterwards we get the
    // first move we legally make. And then we continue making a minimax tree, while checking if we
    // still got time left and if we haven't gotten the stop_flag yet.
    let board = &go_options.board;

    let move_gen = MoveGen::new_legal(board);
    let mut moves: Vec<(isize, ChessMove)> = Vec::new();
    let mut lowest_score_index: usize = 0;

    for child in move_gen {
        let child_board = board.make_move_new(child);
        let score = minimax(&child_board, !board.side_to_move(), 3);

        moves.push((score, child));

        if moves[lowest_score_index].0 >= score {
            lowest_score_index = moves.len() - 1;
        }
    }

    let calculated_move = moves[lowest_score_index].1;

    calculated_move
}

fn minimax(board: &Board, maximizing: Color, depth: usize) -> isize {
    if depth == 0 {
        return evaluate(board, maximizing)
    }

    let mut max = NEG_INFINITY as isize;
    let move_gen = MoveGen::new_legal(board);

    for chess_move in move_gen {
        let new_board = board.make_move_new(chess_move);
        let score = minimax(&new_board, !maximizing, depth - 1);

        max = max.max(score);
    }

    max
}

// for our static evaluation we're counting the number of pieces and castle_rights, each castle right is equal to 100 centipawns (1 pawn) at the moment
fn evaluate(board: &Board, maximizing: Color) -> isize {
    let mut score = 0;
    let max_castle_rights = board.castle_rights(maximizing);
    let min_castle_rights = board.castle_rights(!maximizing);

    score += castle_rights_to_score(max_castle_rights);
    score -= castle_rights_to_score(min_castle_rights);

    let max_pieces = board.color_combined(maximizing);
    let min_pieces = board.color_combined(!maximizing);

    for piece in ALL_PIECES {
        let max_bit_board = board.pieces(piece) & max_pieces;

        score += max_bit_board.popcnt() as isize * piece_to_score(piece);

        let min_bit_board = board.pieces(piece) & min_pieces;

        score -= min_bit_board.popcnt() as isize * piece_to_score(piece);
    }

    score
}

fn castle_rights_to_score(rights: CastleRights) -> isize {
    let original_score = rights.to_index() as isize;

    if original_score >= 2 {
        return (original_score - 1) * 100
    }

    original_score * 100
}

fn piece_to_score(piece: Piece) -> isize {
    match piece {
        Piece::Pawn => 100,
        Piece::Knight => 300,
        Piece::Bishop => 300,
        Piece::Rook => 500,
        Piece::Queen => 900,
        Piece::King => 10000,
    }
}
