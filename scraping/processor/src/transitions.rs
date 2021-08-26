use std::fmt::{self, Display, Formatter};

use crate::{GameAction, GameExt, Replay};
use common::model::Game;

// A transition contains the start and end frames
// For ML, the start is the initial state
// and the end is the desired state
pub struct Transition {
    pub start: Game,
    pub end: Game,
    pub actions: Vec<GameAction>,
}
impl Display for Transition {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let game_1 = format!("{}", self.start);
        let game_2 = format!("{}", self.end);
        for (i, (a, b)) in game_1.lines().zip(game_2.lines()).enumerate() {
            let sep = if i == 10 { "=>    " } else { "      " };
            writeln!(f, "{: <24}{}{}", a, sep, b)?;
        }
        writeln!(f, "{:?}", self.actions)
    }
}

// A transition chain contains a list of transitions
pub struct TransitionChain {
    pub name: String,
    pub transitions: Vec<Transition>,
}

pub fn replay_to_transition_chain(replay: &Replay) -> TransitionChain {
    let mut game_queue = replay.queue.clone();
    let mut game = Game::from_long_queue(&mut game_queue);
    let mut transitions = Vec::new();

    let mut start = game;
    let mut actions = Vec::new();
    for action in replay.actions.iter().map(|x| *x) {
        match action {
            GameAction::AddGarbage { .. } => {
                actions.clear();
                game.apply_action(action);
                start = game;
            }
            GameAction::HardDrop => {
                let mut end = game.clone();
                end.apply_action(GameAction::SoftDrop);

                // Remove any trailing shiftDown or softDrop from actions
                while let Some(GameAction::ShiftDown | GameAction::SoftDrop) = actions.last() {
                    actions.pop();
                }
                actions.push(GameAction::SoftDrop);

                transitions.push(Transition {
                    start,
                    end,
                    actions: actions.clone(),
                });
                actions.clear();
                game.apply_action(action);
                start = game;
            }
            action => {
                actions.push(action);
                game.apply_action(action);
            }
        }
        game.refill_long_queue(&mut game_queue);
    }
    let name = replay.name.clone();
    TransitionChain { name, transitions }
}
