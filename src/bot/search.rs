use {
    crate::bot::{
        GoOptions,
    },
    chess::{
        ChessMove,
        MoveGen,
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
    let mut move_gen = MoveGen::new_legal(&go_options.board);
    let calculated_move = move_gen.next().unwrap();

    calculated_move
}
