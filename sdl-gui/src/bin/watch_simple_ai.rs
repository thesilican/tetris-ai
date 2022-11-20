use common::SimpleAi;
use sdl_gui::Gui;

fn main() {
    Gui::new().watch(SimpleAi::new());
}
