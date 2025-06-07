use fexun_logging::start_logging;
use fexun_winit::start_fexun;

pub mod prelude {
    pub use crate::game;
    pub use crate::run;
}

pub fn run(game: fn()) {
    game();
}
pub fn game() {
    start_logging(); // Starts the logging system
    start_fexun(); // Spins up the Event Loop (Along with Winit and the renderer)
}
