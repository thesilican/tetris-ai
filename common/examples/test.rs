#![feature(error_iter)]
use common::{misc::*, model::GameMove};
use std::{error::Error, hash::Hash, num::ParseIntError};

fn main() {
    let moves = vec![GameMove::ShiftLeft];
    let moves_b = vec![GameMove::ShiftLeft];
    dbg!(moves == moves_b);
}
