use common::api::ai::TetrisAI;
use rusty_ai::ai::RustyAI;
// use rusty_ai::ai_weights::AIWeights;

fn main() {
    let weights = "PYwapDu/O/g+huAmPnB8hD4AE6C9S2mWvlBLTb5gupa9sl4rvik9BL40dga+jkoZvoYGq749vn++BcGZvk//575PwAy+FDbivnahs75kMNG+JRAZvoRS6b5t6JG+mBLwPM+xvD1PMm098e4DPfmL4j3d1Yk9OT/hOr73kr2fpo29N1RGvCFNSA==".parse().unwrap();
    // weights.values = [
    //     1.0, // PC
    //     1.0, // 1 Line
    //     1.0, // 2 Line
    //     1.0, // 3 Line
    //     1.0, // 4 Line
    //     -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, // Holes
    //     -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, // Height
    //     0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, // Height Delta
    // ];
    RustyAI::new(&weights, 20).watch_ai(10);
}
