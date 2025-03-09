use anyhow::Result;
use sdl_gui::AiGui;
use tree_bot::{TreeAi, DEFAULT_PARAMS};

fn main() -> Result<()> {
    AiGui::new(TreeAi::new(4, 10, DEFAULT_PARAMS))?.run()?;
    Ok(())
}
