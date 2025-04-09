pub mod modinit;

pub mod prelude {
    pub use crate::game;
    pub use crate::run;
}

pub fn run(game: fn()) {
    game();
}
pub fn game() {
    modinit::start();
}
