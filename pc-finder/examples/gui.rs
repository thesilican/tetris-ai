use pc_finder::PcFinderAi;
use sdl_gui::Gui;

fn main() {
    Gui::new().watch(PcFinderAi::new());
}
