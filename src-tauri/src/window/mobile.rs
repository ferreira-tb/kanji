use anyhow::Result;
use tauri::{AppHandle, WebviewWindowBuilder};

pub fn open(app: &AppHandle) -> Result<()> {
  WebviewWindowBuilder::new(app, "main", super::url())
    .initialization_script(super::script())
    .build()?;

  Ok(())
}
