use common::api::Ai;
use ml_bot::MlBot;

fn main() {
    let mut ml_bot = MlBot::new();
    ml_bot.watch_ai(1234);
}
