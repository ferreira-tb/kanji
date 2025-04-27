use crate::window::WindowExt;
use anyhow::Result;
use tauri::Wry;
use tauri::plugin::TauriPlugin;

pub fn pinia() -> TauriPlugin<Wry> {
  tauri_plugin_pinia::Builder::new()
    .pretty(true)
    .build()
}

pub fn prevent_default() -> TauriPlugin<Wry> {
  use tauri_plugin_prevent_default::{Builder, Flags, WindowsOptions};

  Builder::new()
    .with_flags(Flags::debug())
    .platform(WindowsOptions {
      general_autofill: false,
      password_autosave: false,
    })
    .build()
}

pub fn single_instance() -> TauriPlugin<Wry> {
  tauri_plugin_single_instance::init(|app, _, _| {
    let window = app.main_window();
    let _: Result<()> = try {
      window.show()?;
      window.unminimize()?;
      window.set_focus()?;
    };
  })
}

pub fn window_state() -> TauriPlugin<Wry> {
  use tauri_plugin_window_state::StateFlags as Flags;

  tauri_plugin_window_state::Builder::new()
    .with_state_flags(Flags::MAXIMIZED | Flags::POSITION | Flags::SIZE)
    .build()
}
