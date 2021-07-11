use common::model::{Bag, Game};

fn main() {
    let mut bag = Bag::new(0);
    let mut game = Game::from_bag(&mut bag, true);
    let children = game.child_states_nr();
    let children = game.child_states_sr();
    let children = game.child_states_dr();
}
