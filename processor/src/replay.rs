use crate::{frames::FrameCollection, GameExt};
use libtetris::model::{Action, Game, Stream, BOARD_HEIGHT, BOARD_WIDTH};
use std::collections::HashSet;
use std::fmt::Write;
use std::fmt::{self, Display, Formatter};
use std::iter::FromIterator;
use std::lazy::{Lazy, OnceCell};

// Keyframes are basically the first and last frame in between hard-drops
// start is the frame directly after a hard drop and subsiquent garbage
// end is the frame after all normal moves and before the first hard drop
//      after the start frame
// It is possible for start and end to be equal, if no moves were made
//      between two hard drops
#[derive(Debug, Clone)]
pub struct KeyFrame {
    pub start: Game,
    pub end: Game,
    pub actions: Vec<Action>,
}
impl Display for KeyFrame {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", write_two_games(self.start, self.end))?;
        writeln!(f, "{:?}", self.actions)?;
        Ok(())
    }
}

// Replay contains all the necessary information to recreate
// a tetris game
#[derive(Debug, Clone)]
pub struct Replay {
    pub name: String,
    pub stream: Stream,
    pub actions: Vec<Action>,
    frames_cache: OnceCell<Vec<Game>>,
    keyframes_cache: OnceCell<Vec<KeyFrame>>,
}
impl Replay {
    pub fn from_frame_collection(frames: &FrameCollection) -> Self {
        println!("Converting frames {} to replay...", frames.name);
        let stream = frames_to_stream(frames);
        let actions = frames_stream_to_actions(frames, &stream);
        Replay {
            name: frames.name.clone(),
            stream,
            actions,
            frames_cache: OnceCell::new(),
            keyframes_cache: OnceCell::new(),
        }
    }
    pub fn frames(&self) -> &Vec<Game> {
        self.frames_cache.get_or_init(|| replay_to_frames(self))
    }
    pub fn keyframes(&self) -> &Vec<KeyFrame> {
        self.keyframes_cache
            .get_or_init(|| replay_to_keyframes(self))
    }
}

// Utility function to print two games side by side
fn write_two_games(game_1: Game, game_2: Game) -> String {
    let game_1 = format!("{}", game_1);
    let game_2 = format!("{}", game_2);
    let mut out = String::new();
    for (i, (a, b)) in game_1.lines().zip(game_2.lines()).enumerate() {
        let sep = if i == 10 { "=>    " } else { "      " };
        writeln!(out, "{: <24}{}{}", a, sep, b).unwrap();
    }
    out
}

// Extract the queue from a game replay
fn frames_to_stream(frames: &FrameCollection) -> Stream {
    // Sanity check
    for frame in &frames.frames {
        assert!(frame.queue_pieces.len() >= 5)
    }
    let first_frame = frames.frames[0];
    // Handle edge case
    for row in first_frame.board.matrix.iter() {
        assert!(*row == 0);
    }
    let mut pieces = if let Some(hold) = first_frame.hold_piece {
        vec![hold, first_frame.current_piece.piece_type]
    } else {
        vec![first_frame.current_piece.piece_type]
    };
    let mut frame_iter = frames.frames.iter();

    // Start with the first frame
    let mut prev_queue = frame_iter
        .next()
        .unwrap()
        .queue_pieces
        .iter()
        .take(5)
        .map(|x| *x)
        .collect::<Vec<_>>();

    for frame in frame_iter {
        let queue = frame
            .queue_pieces
            .iter()
            .take(5)
            .map(|x| *x)
            .collect::<Vec<_>>();
        // Keep popping from prev_queue until they match
        while !queue.iter().zip(prev_queue.iter()).all(|(a, b)| a == b) {
            pieces.push(prev_queue.remove(0))
        }
        prev_queue = queue;
    }
    pieces.append(&mut prev_queue);

    Stream::from_iter(pieces)
}

static ACTIONS_LIST: Lazy<Vec<Vec<Action>>> = Lazy::new(|| {
    let pool = vec![
        // No-op
        vec![],
        // Hold
        vec![Action::Hold],
        // Rotates
        vec![Action::RotateCW],
        vec![Action::Rotate180],
        vec![Action::RotateCCW],
        // Shifts left/right
        vec![Action::ShiftLeft; 1],
        vec![Action::ShiftRight; 1],
        vec![Action::ShiftLeft; 2],
        vec![Action::ShiftRight; 2],
        vec![Action::ShiftLeft; 3],
        vec![Action::ShiftRight; 3],
        vec![Action::ShiftLeft; 7],
        vec![Action::ShiftRight; 4],
        vec![Action::ShiftLeft; 4],
        vec![Action::ShiftRight; 5],
        vec![Action::ShiftLeft; 5],
        vec![Action::ShiftRight; 6],
        vec![Action::ShiftLeft; 6],
        vec![Action::ShiftRight; 7],
        vec![Action::ShiftLeft; 8],
        vec![Action::ShiftRight; 8],
        vec![Action::ShiftLeft; 9],
        vec![Action::ShiftRight; 9],
        vec![Action::ShiftLeft; 10],
        vec![Action::ShiftRight; 10],
        // Shift down
        vec![Action::ShiftDown],
        vec![Action::ShiftDown; 2],
        vec![Action::ShiftDown; 3],
        vec![Action::ShiftDown; 4],
        vec![Action::ShiftDown; 5],
        vec![Action::ShiftDown; 6],
        vec![Action::ShiftDown; 7],
        vec![Action::ShiftDown; 8],
        vec![Action::ShiftDown; 9],
        vec![Action::ShiftDown; 10],
        // Soft drop
        vec![Action::SoftDrop],
        // Hard drop
        vec![Action::SoftDrop, Action::Lock],
    ];
    let mut actions_list = Vec::<Vec<Action>>::new();
    let mut actions_set = HashSet::<Vec<Action>>::new();
    for action_1 in pool.iter() {
        for action_2 in pool.iter() {
            for action_3 in pool.iter() {
                for action_4 in pool.iter() {
                    let mut actions = Vec::<Action>::new();
                    actions.extend(action_1);
                    actions.extend(action_2);
                    actions.extend(action_3);
                    actions.extend(action_4);
                    // Ensure that there is at most 1 Action::Lock
                    let lock_count = actions.iter().filter(|x| matches!(x, Action::Lock)).count();
                    if lock_count > 1 {
                        continue;
                    }

                    if actions_set.insert(actions.clone()) {
                        actions_list.push(actions);
                    }
                }
            }
        }
    }
    actions_list.sort_by_key(|a| a.len());
    actions_list
});

// Find a list of actions to get from each frame to the next
// All of these actions strung together form the game actions for the replay
fn frames_stream_to_actions(frames: &FrameCollection, stream: &Stream) -> Vec<Action> {
    // Find the garbage actions that convert curr => target
    // or None if not possible
    fn find_garbage_actions(curr: Game, target: Game) -> Option<Vec<Action>> {
        // Find the height of the garbage
        // i.e. find the first height where for the board matrixes:
        // target[j..] == curr[?..]
        let height = (0..BOARD_HEIGHT as usize).into_iter().find(|j| {
            target.board.matrix[*j..]
                .iter()
                .zip(curr.board.matrix.iter())
                .all(|(a, b)| *a == *b)
        })?;

        // Determine the col of each garbage row
        // j in 0..height => i where i is the hole in row j
        let garbage_cols = (0..height)
            .into_iter()
            .map(|j| {
                let row = target.board.matrix[j];
                let garbage_col = (0..BOARD_WIDTH).into_iter().find(|col| {
                    let compare = !(1 << col) & ((1 << BOARD_WIDTH) - 1);
                    row == compare
                });
                garbage_col
            })
            .rev()
            .collect::<Option<Vec<_>>>()?;

        // Turn the garbage cols into actions
        let garbage_actions = garbage_cols
            .into_iter()
            .fold(Vec::<Action>::new(), |mut a, v| {
                // If the previous garbage is in the same column,
                // simply increase the height
                if let Some(Action::AddGarbage {
                    col,
                    ref mut height,
                }) = a.last_mut()
                {
                    if *col == v {
                        *height += 1;
                        return a;
                    }
                }
                // Otherwise add a new garbage column
                a.push(Action::AddGarbage { col: v, height: 1 });
                a
            });
        Some(garbage_actions)
    }

    let mut all_actions = Vec::<Action>::new();
    let mut curr = Game::from_stream(&mut stream.clone());
    // Windows iterate over all pairs of frames
    for target in frames.frames.iter() {
        let target = *target;
        // Find the first actions in ACTIONS_LIST
        // where find_garbage_actions returns a valid result
        let actions = ACTIONS_LIST.iter().find_map(|actions| {
            let mut game = curr;
            let mut actions_final = Vec::new();
            // Debug
            for action in actions.iter() {
                game.apply(*action);
                actions_final.push(*action);
                if let Action::Lock = action {
                    let garbage_actions = find_garbage_actions(game, target);
                    if let Some(garbage_actions) = garbage_actions {
                        for action in garbage_actions.iter() {
                            game.apply(*action);
                        }
                        actions_final.extend(garbage_actions);
                    }
                }
            }
            if game.eq_ignore_queue(target) {
                Some(actions_final)
            } else {
                None
            }
        });
        match actions {
            Some(actions) => all_actions.extend(actions),
            None => panic!(
                "Unable to find transition between:\n{}",
                write_two_games(curr, target)
            ),
        }
        curr = target;
    }
    all_actions
}

fn replay_to_keyframes(replay: &Replay) -> Vec<KeyFrame> {
    let mut stream = replay.stream.clone();
    let mut keyframes = Vec::new();

    let mut game = Game::from_stream(&mut stream);
    let mut start = game;
    let mut actions = Vec::new();
    for action in replay.actions.iter() {
        match action {
            Action::Lock => {
                keyframes.push(KeyFrame {
                    start,
                    end: game,
                    actions,
                });
                actions = Vec::new();
                game.apply(*action);
                start = game;
            }
            Action::AddGarbage { .. } => {
                game.apply(*action);
                start = game;
            }
            _ => {
                game.apply(*action);
                actions.push(*action);
            }
        }
        game.refill_queue_stream(&mut stream);
    }
    keyframes
}

fn replay_to_frames(replay: &Replay) -> Vec<Game> {
    let mut queue = replay.stream.clone();
    let mut frames = Vec::new();
    let mut game = Game::from_stream(&mut queue);
    frames.push(game);
    for action in replay.actions.iter() {
        game.apply(*action);
        game.refill_queue_stream(&mut queue);
        frames.push(game);
    }
    frames
}
