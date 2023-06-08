use anyhow::Result;
use deep_bot::DeepAi;
use sdl_gui::AiGui;

fn main() -> Result<()> {
    AiGui::new(DeepAi::new(5, 6))?.run()?;
    Ok(())
}
