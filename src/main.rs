#![allow(unused)]
mod guessing_game;
mod temp_converter;

use crate::guessing_game::guessing_game;
use crate::temp_converter::temp_converter;
fn main() {
    // guessing_game();

    temp_converter(30.4, 'C');
}
