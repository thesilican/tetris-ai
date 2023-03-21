use anyhow::Result;
use sdl_gui::*;

fn main() -> Result<()> {
    PlayGui::new()?.run()
}
