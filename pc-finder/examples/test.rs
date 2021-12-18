use common::*;
use pc_finder::*;

fn main() {
    let board = PcBoard::from_rows([0b0000001111, 0b0000010000, 0b0000100000, 0b0000011000]);
    println!("{:#}", board);
    println!("{}", board.is_valid());
    // println!("{:?}", graph);
}
