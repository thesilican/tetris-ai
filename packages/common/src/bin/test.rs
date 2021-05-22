use common::model::computed::{PieceInfo, PIECE_INFO};
use common::model::game::Game;

fn main() {
    println!("{}", std::mem::size_of::<Game>());
    println!("{}", std::mem::size_of::<PieceInfo>())
}
