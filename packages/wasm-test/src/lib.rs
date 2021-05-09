use common::model::game::{Game, GameMove};
use common::model::piece::Piece;
use common::model::piece::PieceType;
use wasm_bindgen::prelude::*;
use web_sys::console;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    let mut game = Game::new();
    for piece_type in PieceType::iter_types() {
        game.append_queue(Piece::new(&piece_type));
    }
    for _ in 0..5 {
        game.make_move(&GameMove::HardDrop);
    }
    console::log_1(&JsValue::from_str(&format!("{}", game)));
    Ok(())
}
