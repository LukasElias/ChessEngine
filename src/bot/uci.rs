use {
    std::str::SplitWhitespace,
    crate::EngineError,
};

// Everything that will have arguments after the UCI command should have a SplitWhitespace struct for the arguments so it can parse it all and make it easier for the listen method to call each function
pub trait UCI {
    fn listen(&mut self) -> Result<(), EngineError>;
    fn uci(&self) -> Result<(), EngineError>;
    // fn debug(&self, on: bool) -> Result<(), EngineError>;
    fn isready(&self) -> Result<(), EngineError>;
    // fn setoption(&self, ) -> Result<(), EngineError>;
    // fn register(&self) -> Result<(), EngineError>;
    fn ucinewgame(&mut self) -> Result<(), EngineError>;
    fn position(&mut self, arguments: &mut SplitWhitespace) -> Result<(), EngineError>;
    fn go(&self, arguments: &mut SplitWhitespace) -> Result<(), EngineError>;
    // fn stop(&self) -> Result<(), EngineError>;
    // fn ponderhit(&self) -> Result<(), EngineError>;
    // fn quit(&self) -> Result<(), EngineError>;
}

// #[derive(Clone, Debug, Eq, PartialEq, Hash, Default)]
// pub struct GoOptions {
//     pub search_moves: Vec<ChessMove>,
//     pub ponder: bool,
//     pub white_time: Option<Duration>,
//     pub black_time: Option<Duration>,
//     pub white_increment_time: Duration,
//     pub black_increment_time: Duration,
//     pub moves_to_go: usize,
//     pub depth: usize,
//     pub nodes: usize,
//     pub mate: usize,
//     pub move_time: MoveTime,
// }
//
// #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Default)]
// pub enum MoveTime {
//     #[default]
//     NotSpecified,
//     Finite(Duration),
//     Infinite,
// }
