
mod cli;
mod clip;
use std::error::Error;
use clip::clip::ClipManager;

fn main() -> Result<(), Box<dyn  Error>>{

    let mut clip_manager = ClipManager::new();

    clip_manager.run()?;

    Ok(())
}
