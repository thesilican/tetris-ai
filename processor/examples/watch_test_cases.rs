use common::misc::GenericErr;
use common::model::{BOARD_HEIGHT, BOARD_WIDTH};
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
        .take(1)
        .map(|frames| Replay::from_frame_collection(frames))
        .map(|replay| TestCase::from_replay(&mut rng, &replay))
        .fold(Vec::new(), |mut a, v| {
            a.extend(v);
            a
        });
    let mut index = 0;

    fn print_test_case(index: usize, test_case: TestCase) {
        let mut board_string = String::new();
        for j in (0..BOARD_HEIGHT).rev() {
            for i in 0..BOARD_WIDTH {
                if test_case.board.get(i, j) {
                    board_string.push_str("[]");
                } else {
                    board_string.push_str("██");
                }
            }
            board_string.push('\n');
        }
        println!(
            "{}\nIndex: {} Label: {}",
            board_string, index, test_case.label
        );
    }

    let stdin = std::io::stdin();
    let stdout = std::io::stdout().into_raw_mode()?;
    stdout.suspend_raw_mode()?;
    print_test_case(index, training_data[index]);

    stdout.activate_raw_mode()?;
    for key in stdin.keys() {
        stdout.suspend_raw_mode()?;
        match key.unwrap() {
            Key::Left => {
                index = (index + training_data.len() - 1) % training_data.len();
            }
            Key::Right => {
                index = (index + 1) % training_data.len();
            }
            Key::Ctrl('c') => {
                return Ok(());
            }
            _ => {}
        }
        print_test_case(index, training_data[index]);
        stdout.activate_raw_mode()?;
    }
    Ok(())
}
