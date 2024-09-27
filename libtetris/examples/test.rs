use libtetris::{Bag, Board, Game, PieceType, FIN_PERMUTATIONS};

fn main() {
    println!("{}", std::mem::size_of::<Board>());
    println!("{}", std::mem::size_of::<Game>());
}
