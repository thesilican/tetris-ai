use crate::{frames::FrameCollection, game_ext::LongQueue, GameAction, GameExt};
use common::model::{
    Game, GameMove, PieceType, BOARD_HEIGHT, BOARD_WIDTH, FRAGMENT_FINAL, FRAGMENT_HOLD,
    FRAGMENT_ROT, FRAGMENT_SHIFT, MOVES_4F,
};
use std::collections::HashSet;
use std::fmt::{self, Display, Formatter};
use std::lazy::SyncLazy;
use std::{collections::VecDeque, convert::TryInto, iter::FromIterator};

#[derive(Debug, Clone)]
pub struct Replay {
    pub name: String,
    pub queue: LongQueue,
    pub actions: Vec<GameAction>,
    pub keyframes: Vec<KeyFrame>,
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
        let game_1 = format!("{}", self.start);
        let game_2 = format!("{}", self.end);
        for (i, (a, b)) in game_1.lines().zip(game_2.lines()).enumerate() {
            let sep = if i == 10 { "=>    " } else { "      " };
            writeln!(f, "{: <24}{}{}", a, sep, b)?;
        }
        Ok(())
    }
}

// Extracts the keyframes from a frame collection
fn frames_to_keyframes(frames: &FrameCollection) -> Vec<KeyFrame> {
    let mut keyframes = Vec::new();
    let mut first_frame = frames.frames[0];
    let mut prev_frame = frames.frames[0];
    for frame in &frames.frames {
        let frame = *frame;
        if frame.board != prev_frame.board {
            keyframes.push(KeyFrame {
                start: first_frame,
                end: prev_frame,
            });
            first_frame = frame;
        }
        prev_frame = frame;
    }
    keyframes
}

static ACTIONS: SyncLazy<Vec<Vec<GameAction>>> = SyncLazy::new(|| {
    // TODO: Find a faster way to do this
    let game_actions = vec![
        // No-op
        vec![],
        // Hold
        vec![GameAction::Hold],
        // Rotates
        vec![GameAction::RotateLeft],
        vec![GameAction::Rotate180],
        vec![GameAction::RotateRight],
        // Shifts left/right
        vec![GameAction::ShiftLeft; 5],
        vec![GameAction::ShiftLeft; 4],
        vec![GameAction::ShiftLeft; 3],
        vec![GameAction::ShiftLeft; 2],
        vec![GameAction::ShiftLeft; 1],
        vec![GameAction::ShiftRight; 1],
        vec![GameAction::ShiftRight; 2],
        vec![GameAction::ShiftRight; 3],
        vec![GameAction::ShiftRight; 4],
        vec![GameAction::ShiftRight; 5],
        // Soft drop
        vec![GameAction::SoftDrop],
        // Shift down
        vec![GameAction::ShiftDown; 1],
        vec![GameAction::ShiftDown; 2],
        vec![GameAction::ShiftDown; 3],
        vec![GameAction::ShiftDown; 4],
        vec![GameAction::ShiftDown; 5],
        vec![GameAction::ShiftDown; 6],
        vec![GameAction::ShiftDown; 7],
        vec![GameAction::ShiftDown; 8],
        vec![GameAction::ShiftDown; 9],
        vec![GameAction::ShiftDown; 10],
        vec![GameAction::ShiftDown; 11],
        vec![GameAction::ShiftDown; 12],
        vec![GameAction::ShiftDown; 13],
        vec![GameAction::ShiftDown; 14],
        vec![GameAction::ShiftDown; 15],
    ];
    let mut actions_list = Vec::new();
    let mut actions_set = HashSet::new();
    for action_1 in &game_actions {
        for action_2 in &game_actions {
            for action_3 in &game_actions {
                for action_4 in &game_actions {
                    let mut actions = Vec::<GameAction>::new();
                    actions.extend(action_1);
                    actions.extend(action_2);
                    actions.extend(action_3);
                    actions.extend(action_4);
                    if actions_set.contains(&actions) {
                        continue;
                    }
                    // Filters
                    let mut count_hold = 0;
                    let mut count_soft_drop = 0;
                    for item in actions.iter() {
                        match item {
                            GameAction::SoftDrop => count_soft_drop += 1,
                            GameAction::Hold => count_hold += 1,
                            _ => {}
                        }
                    }
                    if count_soft_drop > 20 || count_hold > 1 {
                        continue;
                    }
                    // Insert
                    actions_set.insert(actions.clone());
                    actions_list.push(actions);
                }
            }
        }
    }
    actions_list.sort_by_key(|v| v.len());
    // println!("actions_list has {} items", actions_list.len());
    actions_list
});

// Method do be kinda thick but whatever
// Replay basically ensures that it is possible to have
// a string of GameActions that intersects at every keyframe
fn keyframes_queue_to_replay(name: String, keyframes: Vec<KeyFrame>, queue: LongQueue) -> Replay {
    // Test that two games are equal
    // If queues aren't same length, test the shortest of the twos
    fn games_eq(game_1: &Game, game_2: &Game) -> bool {
        game_1.board == game_2.board
            && game_1.current_piece == game_2.current_piece
            && game_1.hold_piece == game_2.hold_piece
            && game_1.can_hold == game_2.can_hold
            && game_1
                .queue_pieces
                .iter()
                .zip(game_2.queue_pieces.iter())
                .all(|(a, b)| a == b)
    }
    // Given a game and a target, return a list of actions that will take you
    // from game to target.
    fn find_actions(game: &mut Game, target: &Game) -> impl IntoIterator<Item = GameAction> {
        // No need for hashmap because the first one found is guarenteed to be the shortest
        for actions in ACTIONS.iter() {
            let mut child_game = game.clone();
            for action in actions {
                child_game.apply_action(*action);
            }
            if games_eq(&child_game, target) {
                *game = child_game;
                return actions.iter().map(|x| *x);
            }
        }
        // Unable to find transition
        panic!(
            "Unable to find transition between states:\n{}\n{}",
            game, target
        )
    }
    // Given a game and a target
    // First apply GameMove::HardDrop
    // Then find the appropriate amount of garbage lines needed to transfer from the first state to the last state
    fn find_garbage(game: &mut Game, target: &Game) -> impl IntoIterator<Item = GameAction> {
        // Find the first matrix line where target[j..] == board[..?]
        // a.k.a. the number of lines of garbage added
        game.make_move(GameMove::HardDrop);
        let y = (0..BOARD_HEIGHT as usize)
            .find(|j| {
                (&target.board.matrix[*j..])
                    .iter()
                    .zip(game.board.matrix.iter())
                    .all(|(a, b)| a == b)
            })
            .unwrap_or_else(|| {
                panic!(
                    "Unable to match board to garbage lines:\n{}\n{}",
                    game, target
                )
            });
        // Find out the garbage lines
        let mut garbage_cols = Vec::new();
        for i in 0..y {
            let row = target.board.matrix[i];
            let mut garbage_col = None;
            for col in 0..BOARD_WIDTH {
                let compare = !(1 << col) & ((1 << BOARD_WIDTH) - 1);
                if row == compare {
                    garbage_col = Some(col);
                    break;
                }
            }
            match garbage_col {
                None => {
                    // Something went wrong
                    panic!("Unable to match garbage column {}\n{}", i, game);
                }
                Some(col) => garbage_cols.push(col),
            }
        }
        // Turn the garbage cols into actions
        let garbage_actions =
            garbage_cols
                .into_iter()
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
        for game_action in &garbage_actions {
            game.apply_action(*game_action);
        }
        // Special case, undo shift down
        if game.board.intersects_with(&game.current_piece) {
            game.current_piece.location.1 += 1;
        }
        std::iter::once(GameMove::HardDrop.into()).chain(garbage_actions.into_iter())
    }

    let mut actions = Vec::<GameAction>::new();
    let mut game_queue = queue.clone();
    let mut game = Game::from_long_queue(&mut game_queue);
    game.current_piece.reset(&game.board);
    // println!("{}", game);
    // println!("{}", keyframes[0].start);
    assert!(games_eq(&game, &keyframes[0].start));

    // Start => End, just for the first keyframe
    let mut keyframes_iter = keyframes.iter();
    let KeyFrame { end, .. } = keyframes_iter.next().unwrap();
    actions.extend(find_actions(&mut game, &end));

    // Remaining keyframes..
    for KeyFrame { start, end } in keyframes_iter {
        // Prev end => Start
        actions.extend(find_garbage(&mut game, &start));
        // Start => End
        actions.extend(find_actions(&mut game, &end));
        game.refill_long_queue(&mut game_queue);
        // println!("{}", game);
    }
    Replay {
        actions,
        name,
        queue,
        keyframes,
    }
}

pub fn frame_collection_to_replay(frames: &FrameCollection) -> Replay {
    println!("Converting frame collection ({}) to replay...", frames.name);
    // Start by determining the queue
    let queue = frames_to_queue(frames);
    let keyframes = frames_to_keyframes(frames);
    let name = frames.name.clone();
    keyframes_queue_to_replay(name, keyframes, queue)
}
