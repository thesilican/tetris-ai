use common::*;

fn main() {
    // Assumption: Queue starts with [O] I T L ...
    let moves = vec![
        GameMove::Hold,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::HardDrop,
        GameMove::Hold,
        GameMove::RotateCW,
        GameMove::ShiftRight,
        GameMove::HardDrop,
        GameMove::HardDrop,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::HardDrop,
        GameMove::HardDrop,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::HardDrop,
        GameMove::Hold,
        GameMove::HardDrop,
        GameMove::Hold,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::HardDrop,
        GameMove::Hold,
        GameMove::RotateCW,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::HardDrop,
        GameMove::RotateCCW,
        GameMove::ShiftLeft,
        GameMove::HardDrop,
        GameMove::RotateCW,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::HardDrop,
        GameMove::RotateCW,
        GameMove::HardDrop,
        GameMove::RotateCW,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::HardDrop,
        GameMove::Hold,
        GameMove::RotateCW,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::SoftDrop,
        GameMove::RotateCCW,
        GameMove::RotateCCW,
        GameMove::SoftDrop,
        GameMove::RotateCCW,
        GameMove::HardDrop,
        GameMove::RotateCW,
        GameMove::HardDrop,
        GameMove::RotateCW,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::SoftDrop,
        GameMove::RotateCCW,
        GameMove::RotateCCW,
        GameMove::HardDrop,
        GameMove::Hold,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::HardDrop,
        GameMove::Hold,
        GameMove::RotateCCW,
        GameMove::HardDrop,
        GameMove::Hold,
        GameMove::RotateCW,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::HardDrop,
        GameMove::RotateCW,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::HardDrop,
        GameMove::Hold,
        GameMove::RotateCW,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::SoftDrop,
        GameMove::RotateCW,
        GameMove::HardDrop,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::HardDrop,
        GameMove::Hold,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::HardDrop,
        GameMove::RotateCW,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::SoftDrop,
        GameMove::ShiftRight,
        GameMove::HardDrop,
        GameMove::RotateCCW,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::HardDrop,
        GameMove::RotateCW,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::HardDrop,
        GameMove::HardDrop,
        GameMove::Hold,
        GameMove::RotateCCW,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::SoftDrop,
        GameMove::RotateCCW,
        GameMove::HardDrop,
        GameMove::RotateCCW,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::HardDrop,
        GameMove::RotateCCW,
        GameMove::ShiftRight,
        GameMove::SoftDrop,
        GameMove::HardDrop,
        GameMove::Hold,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::HardDrop,
        GameMove::RotateCW,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::HardDrop,
        GameMove::ShiftLeft,
        GameMove::SoftDrop,
        GameMove::HardDrop,
        GameMove::RotateCW,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::SoftDrop,
        GameMove::RotateCCW,
        GameMove::HardDrop,
        GameMove::Hold,
        GameMove::Rotate180,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::HardDrop,
    ];
    let mut bag = Bag::new_fixed(&PieceType::ALL);
    let mut game = Game::from_bag(&mut bag);
    game.swap_hold();
    for _ in 0..10_000 {
        for &game_move in moves.iter() {
            game.make_move(game_move);
            game.refill_queue(&mut bag);
            println!("{game}");
            std::thread::sleep(std::time::Duration::from_millis(50));
        }
    }
}
