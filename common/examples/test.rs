use common::model::{Bag, ChildState, Fragment, Fragments, Game, GameMove, Stream, MOVES_4F};

fn main() {
    let mut bag = Bag::new(9);
    let game = Game::from_bag_shuffled(&mut bag);
    println!("{}", std::mem::size_of_val(&game));
    let child_states = game.child_states(&MOVES_4F);
    // for ChildState { game, moves } in &child_states {
    //     println!("{}\n{:?}", game, moves);
    // }
    println!("{}", child_states.len());
}
