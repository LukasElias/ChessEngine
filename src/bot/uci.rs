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
