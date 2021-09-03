use crate::game_ext::GameActionRes;
use crate::{frames::FrameCollection, game_ext::LongQueue, GameAction, GameExt};
use common::model::{
    Game, GameMove, PieceType, BOARD_HEIGHT, BOARD_WIDTH, FRAGMENT_FINAL, FRAGMENT_HOLD,
    FRAGMENT_ROT, FRAGMENT_SHIFT, MOVES_4F,
};
use std::collections::HashSet;
use std::fmt::Write;
use std::fmt::{self, Display, Formatter};
use std::lazy::SyncLazy;
use std::{collections::VecDeque, convert::TryInto, iter::FromIterator};

// Keyframes are basically the first and last frame in between hard-drops
// start is the frame directly after a hard drop and subsiquent garbage
// end is the frame after all normal moves and before the first hard drop
//      after the start frame
// It is possible for start and end to be equal, if no moves were made
//      between two hard drops
#[derive(Debug, Clone)]
pub struct KeyFrame {
    start: Game,
    end: Game,
}
impl Display for KeyFrame {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", write_two_games(self.start, self.end))
    }
}

// Replay contains all the necessary information to recreate
// a tetris game
#[derive(Debug, Clone)]
pub struct Replay {
    pub name: String,
    pub queue: LongQueue,
    pub actions: Vec<GameAction>,
    frames_cache: Option<Vec<Game>>,
    keyframes_cache: Option<Vec<KeyFrame>>,
}
impl Replay {
    pub fn from_frame_collection(frames: &FrameCollection) -> Self {
        println!("Converting frame collection ({}) to replay...", frames.name);
        let queue = frames_to_queue(frames);
        let actions = frames_to_actions(frames);
        Replay {
            name: frames.name.clone(),
            queue,
            actions,
            frames_cache: None,
            keyframes_cache: None,
        }
    }
    pub fn frames(&mut self) -> &Vec<Game> {
        if let None = self.frames_cache {
            self.frames_cache = Some(replay_to_frames(self));
        }
        self.frames_cache.as_ref().unwrap()
    }
    pub fn keyframes(&mut self) -> &Vec<KeyFrame> {
        if let None = self.keyframes_cache {
            self.keyframes_cache = Some(replay_to_keyframes(self));
        }
        self.keyframes_cache.as_ref().unwrap()
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
fn frames_to_queue(frames: &FrameCollection) -> LongQueue {
    // Sanity check
    for frame in &frames.frames {
        assert!(frame.queue_pieces.len() >= 5)
    }
    let mut pieces = vec![frames.frames[0].current_piece.piece_type];
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

    LongQueue::from_iter(pieces)
}

static ACTIONS_LIST: SyncLazy<Vec<Vec<GameAction>>> = SyncLazy::new(|| {
    let pool = vec![
        // No-op
        vec![],
        // Hold
        vec![GameAction::Hold],
        // Rotates
        vec![GameAction::RotateLeft],
        vec![GameAction::RotateRight],
        vec![GameAction::Rotate180],
        // Soft drop
        vec![GameAction::SoftDrop],
        // Shift down
        vec![GameAction::ShiftDown],
        // Shifts left/right
        vec![GameAction::ShiftLeft; 1],
        vec![GameAction::ShiftRight; 1],
        vec![GameAction::ShiftLeft; 2],
        vec![GameAction::ShiftRight; 2],
        vec![GameAction::ShiftLeft; 3],
        vec![GameAction::ShiftRight; 3],
        vec![GameAction::ShiftLeft; 7],
        vec![GameAction::ShiftRight; 4],
        vec![GameAction::ShiftLeft; 4],
        vec![GameAction::ShiftRight; 5],
        vec![GameAction::ShiftLeft; 5],
        vec![GameAction::ShiftRight; 6],
        vec![GameAction::ShiftLeft; 6],
        vec![GameAction::ShiftRight; 7],
        vec![GameAction::ShiftLeft; 8],
        vec![GameAction::ShiftRight; 8],
        vec![GameAction::ShiftLeft; 9],
        vec![GameAction::ShiftRight; 9],
        vec![GameAction::ShiftLeft; 10],
        vec![GameAction::ShiftRight; 10],
        // Hard drop
        vec![GameAction::HardDrop],
    ];
    let mut actions_list = Vec::<Vec<GameAction>>::new();
    let mut actions_set = HashSet::<Vec<GameAction>>::new();
    for action_1 in pool.iter() {
        for action_2 in pool.iter() {
            'l: for action_3 in pool.iter() {
                let mut actions = Vec::<GameAction>::new();
                actions.extend(action_1);
                actions.extend(action_2);
                actions.extend(action_3);
                // Checks
                for (i, action) in actions.iter().enumerate() {
                    if let GameAction::HardDrop = action {
                        // GameAction::HardDrop must be the last element of actions
                        if i != actions.len() {
                            continue 'l;
                        }
                    }
                }
                if actions_set.insert(actions.clone()) {
                    actions_list.push(actions);
                }
            }
        }
    }
    actions_list.sort_by_key(|a| a.len());
    actions_list
});

// Find a list of actions to get from each frame to the next
// All of these actions strung together form the game actions for the replay
fn frames_to_actions(frames: &FrameCollection) -> Vec<GameAction> {
    // Find the garbage actions that convert curr => target
    // or None if not possible
    fn find_garbage_actions(curr: Game, target: Game) -> Option<Vec<GameAction>> {
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
        let garbage_cols = (0..height).into_iter().map(|j| {
            let row = target.board.matrix[j];
            let garbage_col = (0..BOARD_WIDTH).into_iter().find(|col| {
                let compare = !(1 << col) & ((1 << BOARD_WIDTH) - 1);
                row == compare
            });
            match garbage_col {
                // Something went wrong
                Some(col) => col,
                None => panic!("Unable to find garbage row {}:\n{}", j, target),
            }
        });

        // Turn the garbage cols into actions
        let garbage_actions = garbage_cols
            .rev()
            .fold(Vec::<GameAction>::new(), |mut a, v| {
                // If the previous garbage is in the same column,
                // simply increase the height
                if let Some(GameAction::AddGarbage {
                    col,
                    ref mut height,
                }) = a.last_mut()
                {
                    if *col == v {
                        *height += 1;
                    }
                } else {
                    // Otherwise add a new garbage column
                    a.push(GameAction::AddGarbage { col: v, height: 1 });
                }
                a
            });
        Some(garbage_actions)
    }

    let mut all_actions = Vec::<GameAction>::new();
    // Windows iterate over all pairs of frames
    for window in frames.frames.windows(2) {
        let (curr, target) = (window[0], window[1]);
        // Find the first actions in ACTIONS_LIST
        // where find_garbage_actions returns a valid result
        let actions = ACTIONS_LIST.iter().find_map(|actions| {
            let mut game = curr;
            for action in actions {
                game.apply_action(*action);
            }
            if let Some(GameAction::HardDrop) = actions.last() {
                // Try finding garbage actions
                let garbage_actions = find_garbage_actions(game, target)?;
                for action in garbage_actions.iter() {
                    game.apply_action(*action);
                }
                assert!(game.eq_ignore_queue(target));
                let all_actions = actions
                    .iter()
                    .chain(garbage_actions.iter())
                    .map(|x| *x)
                    .collect::<Vec<_>>();
                Some(all_actions)
            } else {
                // Just check if they're equal
                if game.eq_ignore_queue(target) {
                    Some(actions.clone())
                } else {
                    None
                }
            }
        });
        match actions {
            Some(actions) => all_actions.extend(actions),
            None => panic!(
                "Unable to find transition between:\n{}",
                write_two_games(curr, target)
            ),
        }
    }
    all_actions
}

fn replay_to_keyframes(replay: &Replay) -> Vec<KeyFrame> {
    let mut queue = replay.queue.clone();
    let mut keyframes = Vec::new();
    let mut game = Game::from_long_queue(&mut queue);
    let mut start = game;
    for action in replay.actions.iter() {
        match action {
            GameAction::HardDrop => {
                keyframes.push(KeyFrame { start, end: game });
                game.apply_action(*action);
                start = game;
            }
            GameAction::AddGarbage { .. } => {
                game.apply_action(*action);
                start = game;
            }
            _ => {
                game.apply_action(*action);
            }
        }
    }
    keyframes
}

fn replay_to_frames(replay: &Replay) -> Vec<Game> {
    let mut queue = replay.queue.clone();
    let mut frames = Vec::new();
    let mut game = Game::from_long_queue(&mut queue);
    frames.push(game);
    for action in replay.actions.iter() {
        game.apply_action(*action);
        game.refill_long_queue(&mut queue);
        frames.push(game);
    }
    frames
}
