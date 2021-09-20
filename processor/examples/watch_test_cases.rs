use std::cell::Cell;

use common::misc::GenericErr;
use common::model::{Game, PieceType};
use processor::{FrameCollection, Replay, TestCase};
use rand::prelude::StdRng;
use rand::SeedableRng;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() -> Result<(), GenericErr> {
    let mut rng = StdRng::seed_from_u64(1234);
    let training_data = FrameCollection::load()
        .iter()
        .map(|frames| Replay::from_frame_collection(frames))
        .map(|replay| TestCase::from_replay(&mut rng, &replay))
        .fold(Vec::new(), |mut a, v| {
            a.extend(v);
            a
        });
    let case = Cell::new(0);
    let num_case = training_data.len();
    let piece = Cell::new(0);
    let num_piece = 8;

    let print_state = || {
        let case = case.get();
        let piece = piece.get();
        println!(
            "{} {} {}",
            case,
            piece,
            piece == training_data[case].expected
        );
        let mut game = Game::from_pieces(PieceType::O, None, &[]);
        game.current_piece = training_data[case].pieces[piece];
        game.board = training_data[case].board;
        println!("{}", game);
    };

    let stdin = std::io::stdin();
    let stdout = std::io::stdout().into_raw_mode()?;
    stdout.suspend_raw_mode()?;
    print_state();

    stdout.activate_raw_mode()?;
    for key in stdin.keys() {
        stdout.suspend_raw_mode()?;
        match key.unwrap() {
            Key::Left => {
                piece.set((piece.get() + num_piece - 1) % num_piece);
            }
            Key::Right => {
                piece.set((piece.get() + 1) % num_piece);
            }
            Key::Up => {
                case.set((case.get() + num_case - 1) % num_case);
                piece.set(0);
            }
            Key::Down => {
                case.set((case.get() + 1) % num_case);
                piece.set(0);
            }
            Key::Ctrl('c') => {
                return Ok(());
            }
            _ => {}
        }
        print_state();
        stdout.activate_raw_mode()?;
    }
    Ok(())
}
