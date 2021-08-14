use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn greet(num: i32) -> String {
    match num {
        0 => String::from("Hello, world!"),
        1 => String::from("What's up, homie?"),
        _ => panic!("at the disco"),
    }
}
pub fn set_panic_hook() {
    console_error_panic_hook::set_once();
}
