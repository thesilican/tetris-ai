use libtetris::{Ai, Evaluation, Game, SimpleAi};
use once_cell::sync::Lazy;
use std::sync::Mutex;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
    log("Initializing web-wasm");
}

#[wasm_bindgen]
pub struct ApiEvaluation {
    success: bool,
    actions: Vec<String>,
    message: String,
}

#[wasm_bindgen]
impl ApiEvaluation {
    #[wasm_bindgen]
    pub fn success(&self) -> bool {
        self.success
    }

    #[wasm_bindgen]
    pub fn actions(&self) -> Vec<String> {
        self.actions.clone()
    }

    #[wasm_bindgen]
    pub fn message(&self) -> String {
        self.message.clone()
    }
}

static SIMPLE_AI: Lazy<Mutex<SimpleAi>> = Lazy::new(|| Mutex::new(SimpleAi::new()));

#[wasm_bindgen]
pub fn evaluate(ai_type: String, game: String) -> ApiEvaluation {
    let game = match serde_json::from_str::<Game>(&game) {
        Ok(game) => game,
        Err(err) => {
            return ApiEvaluation {
                success: false,
                actions: Vec::new(),
                message: format!("Deserializing game failed: {err}"),
            }
        }
    };
    let evaluation: Evaluation = match ai_type.as_str() {
        "simple-ai" => SIMPLE_AI.lock().unwrap().evaluate(&game),
        _ => {
            return ApiEvaluation {
                success: false,
                actions: Vec::new(),
                message: format!("Unknown ai type {ai_type}"),
            }
        }
    };
    match evaluation {
        Evaluation::Success { actions, score } => ApiEvaluation {
            success: true,
            actions: actions
                .into_iter()
                .map(|action| action.to_string())
                .collect(),
            message: match score {
                Some(score) => format!("Score: {score:0.2}"),
                None => format!(""),
            },
        },
        Evaluation::Fail { message } => ApiEvaluation {
            success: false,
            actions: Vec::new(),
            message: format!("Evaluation failed: {message}"),
        },
    }
}
