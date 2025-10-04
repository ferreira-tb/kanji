use crate::window::WindowExt;
use anyhow::Result;
use tauri::Wry;
use tauri::plugin::TauriPlugin;
use tauri_plugin_pinia::PrettyTomlMarshaler;

pub fn pinia() -> TauriPlugin<Wry> {
  tauri_plugin_pinia::Builder::new()
    .marshaler_of("settings", Box::new(PrettyTomlMarshaler))
    .build()
}

pub fn prevent_default() -> TauriPlugin<Wry> {
  use tauri_plugin_prevent_default::{Builder, Flags, PlatformOptions};
  Builder::new()
    .with_flags(Flags::debug())
    .platform(PlatformOptions {
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
    .with_state_flags(Flags::MAXIMIZED | Flags::POSITION)
    .build()
}
