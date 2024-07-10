use anyhow::Result;
use libtetris::SimpleAi;
use sdl_gui::AiGui;

fn main() -> Result<()> {
    let simple_ai = SimpleAi;
    AiGui::new(simple_ai)?.run()?;
    Ok(())
}
