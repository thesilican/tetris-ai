use c4w_bot::*;
use libtetris::*;

fn main() {
    // let mut bag = Bag::new(6);
    // let mut game = Game::from_bag_shuffled(&mut bag);
    // game.make_move(Action::HardDrop);
    // let state = CenterState::from_board(&game.board);
    // println!("{}", game);
    // println!("{:#}", state);
    let mut bag = Bag::new(6);
    let mut game = Game::from_bag_shuffled(&mut bag);
    let state = CenterState::new(0b1010010110100101);
    state.apply_to_board(&mut game.board);
    println!("{}", game);
}
