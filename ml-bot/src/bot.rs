use crate::NeuralNetwork;
use libtetris::{
    api::{Ai, AiRes},
    misc::GenericErr,
    model::{Game, MOVES_4F},
};

pub struct MlBot {
    neural_network: NeuralNetwork,
}
impl MlBot {
    pub fn new(model_path: &str) -> Result<Self, GenericErr> {
        let neural_network = NeuralNetwork::load(model_path)?;
        Ok(MlBot { neural_network })
    }
}
impl Ai for MlBot {
    fn evaluate(&mut self, game: &Game) -> AiRes {
        let child_states = game.child_states(&MOVES_4F);
        // Assign evaluation to each child state
        // then find the maximum child state
        let evaluations = child_states
            .into_iter()
            .map(|child_state| {
                let score = self.neural_network.run_board(&child_state.game.board);
                (score, child_state)
            })
            .max_by(|(s1, _), (s2, _)| s1.partial_cmp(s2).unwrap());
        match evaluations {
            Some((score, child_state)) => AiRes::Success {
                moves: child_state.moves.to_vec(),
                score: Some(score.into()),
            },
            None => AiRes::Fail {
                reason: "No child states".into(),
            },
        }
    }
}
