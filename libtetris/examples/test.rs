use libtetris::{Bag, Game, PieceType, FIN_PERMUTATIONS};

fn main() {
    let mut bag = Bag::new_fixed(&[PieceType::T, PieceType::I]);
    let mut game = Game::from_bag(&mut bag);
    game.board.set(0, 0, true);
    game.board.set(1, 0, true);
    game.board.set(1, 1, true);
    game.board.set(2, 1, true);
    println!("{game}");
    for child in game.children(4) {
        println!("{}\n{:?}", child.game, child.actions().collect::<Vec<_>>());
    }
}
