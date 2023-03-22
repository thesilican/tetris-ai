use common::*;

fn main() {
    let req = r#"{
        "board": [
            [1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0],
            [1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0],
            [1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0],
            [1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0],
            [1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0],
            [1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0],
            [1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 0],
            [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        ],
        "active": {
            "type": "O",
            "rot": 0,
            "loc": [3, 19]
        },
        "hold": null,
        "canHold": true,
        "queue": ["O", "I", "J", "L", "Z"]
    }"#;
    let mut game: Game = serde_json::from_str(req).unwrap();
    println!("{}", std::mem::size_of::<Game>());
    println!("{game}");
    println!("{}", serde_json::to_string(&game).unwrap());
    let res = SimpleAi.evaluate(&game);
    match res {
        AiRes::Success {
            moves,
            score: _score,
        } => {
            // Make moves
            for &game_move in moves.iter() {
                game.make_move(game_move);
            }
            println!("{moves:?}");
            println!("{game}");
            println!("{}", serde_json::to_string(&game).unwrap());
        }
        AiRes::Fail { reason } => {
            println!("{reason}");
            println!("{}", serde_json::to_string(&reason).unwrap());
        }
    }
    // let res = SimpleAi.api_evaluate(req);
    // println!("{}", res);
}
