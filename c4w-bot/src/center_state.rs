use common::*;
use once_cell::sync::Lazy;
use std::{collections::HashMap, fmt::Display};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CenterState {
    state: u16,
}

impl CenterState {
    pub fn new(state: u16) -> Self {
        CenterState { state }
    }
    pub fn from_board(board: &Board) -> Self {
        let mut state = 0;
        for (i, row) in board.matrix.iter().take(4).enumerate() {
            let row = (*row & 0b0001111000) >> 3;
            state |= row << (i * 4);
        }
        CenterState { state }
    }
    pub fn apply_to_board(&self, board: &mut Board) {
        for (i, row) in board.matrix.iter_mut().take(4).enumerate() {
            let bits = (self.state >> i * 4) & 0b1111;
            *row &= 0b1110000111;
            *row |= bits << 3;
        }
    }
    pub fn state(&self) -> u16 {
        self.state
    }
}

impl Display for CenterState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let sep = if f.alternate() { '\n' } else { '/' };
        let mut text = String::new();
        for y in (0..4).rev() {
            if y != 3 {
                text.push(sep);
            }
            for x in 0..4 {
                let bit = self.state & (1 << x + (y * 4));
                if bit == 0 {
                    text.push('.');
                } else {
                    text.push('@');
                }
            }
        }
        write!(f, "{}", text)
    }
}

pub struct CenterChild {
    state: CenterState,
    moves: &'static [GameMove],
}

pub struct CenterTransitions {
    map: HashMap<CenterState, HashMap<PieceType, Vec<CenterChild>>>,
}

impl CenterTransitions {
    pub fn generate() -> Self {
        todo!()
    }
}
