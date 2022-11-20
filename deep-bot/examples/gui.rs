use deep_bot::DeepAi;
use sdl_gui::Gui;

fn main() {
    Gui::new().watch(DeepAi::new(5, 6));
}
