use libtetris::{Ai, Evaluation, Game, SimpleAi};
use pc_finder::{PcFinderAi, PcTable};
use std::sync::{LazyLock, Mutex, OnceLock};
use tree_bot::{TreeAi, DEFAULT_PARAMS};
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
pub fn init_pc_finder(pc_table: &[u8]) -> bool {
    let Ok(table) = PcTable::load(pc_table) else {
        return false;
    };
    let ai = PcFinderAi::new(table);
    PC_FINDER_AI.set(Mutex::new(ai)).ok();
    true
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

static SIMPLE_AI: LazyLock<Mutex<SimpleAi>> = LazyLock::new(|| Mutex::new(SimpleAi::new()));
static TREE_AI: LazyLock<Mutex<TreeAi>> =
    LazyLock::new(|| Mutex::new(TreeAi::new(4, 6, DEFAULT_PARAMS)));
static PC_FINDER_AI: OnceLock<Mutex<PcFinderAi>> = OnceLock::new();

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
        "simple" => SIMPLE_AI.lock().unwrap().evaluate(&game),
        "tree" => TREE_AI.lock().unwrap().evaluate(&game),
        "pc-finder" => match PC_FINDER_AI.get() {
            Some(ai) => ai.lock().unwrap().evaluate(&game),
            None => {
                return ApiEvaluation {
                    success: false,
                    actions: Vec::new(),
                    message: "PC Table not yet loaded".to_string(),
                }
            }
        },
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
            message: format!("Eval: {score:0.2}"),
        },
        Evaluation::Fail { message } => ApiEvaluation {
            success: false,
            actions: Vec::new(),
            message: format!("Evaluation failed: {message}"),
        },
    }
}
