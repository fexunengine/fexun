pub mod prelude {                   //
    pub use crate::run;             //
    pub use crate::game;            //
}                                   // Non functional code here
                                    //
pub fn run(game: fn()) {            // These are just placeholder functions and modules
    game();                         // so Clippy doesn't annoy the shit
}                                   // out of me...
                                    //
pub fn game() {                     //
    println!("Hello, world!");      //
}                                   //
