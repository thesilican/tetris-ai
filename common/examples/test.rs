use common::model::{game::Game, piece::Bag};

fn main() {
    let mut game = Game::new_with_bag(&Bag::new());
    game.extend_bag(&Bag::new());
    println!("{}", game);
    println!("{}", std::mem::size_of::<Game>());
}
