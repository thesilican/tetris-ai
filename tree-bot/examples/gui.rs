use anyhow::Result;
use sdl_gui::AiGui;
use tree_bot::DeepAi;

fn main() -> Result<()> {
    AiGui::new(DeepAi::new(5, 6))?.run()?;
    Ok(())
}
