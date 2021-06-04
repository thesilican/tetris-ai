use common::api::ai::TetrisAI;
use rusty_ai::ai::RustyAI;
use rusty_ai::ai_weights::AIWeights;

fn main() {
    let mut weights = AIWeights::new();
    weights.values = [
        1.0, // PC
        1.0, // 1 Line
        1.0, // 2 Line
        1.0, // 3 Line
        1.0, // 4 Line
        -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, // Holes
        -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, // Height
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, // Height Delta
    ];
    RustyAI::new(&weights, 20).watch_ai(10);
}
