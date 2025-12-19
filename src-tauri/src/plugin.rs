use tauri::Wry;
use tauri::plugin::TauriPlugin;

#[cfg(desktop)]
use crate::window::desktop::WindowExt;

#[cfg(all(desktop, debug_assertions))]
pub fn log() -> TauriPlugin<Wry> {
  use tauri_plugin_log::{Target, TargetKind};

  tauri_plugin_log::Builder::new()
    .clear_targets()
    .target(Target::new(TargetKind::Stderr))
    .target(Target::new(TargetKind::Webview))
    .build()
}

#[cfg(desktop)]
pub fn prevent_default() -> TauriPlugin<Wry> {
  use tauri_plugin_prevent_default::{Builder, Flags, PlatformOptions};

  Builder::new()
    .with_flags(Flags::debug())
    .platform(
      PlatformOptions::new()
        .browser_accelerator_keys(cfg!(debug_assertions))
        .default_context_menus(cfg!(debug_assertions))
        .default_script_dialogs(cfg!(debug_assertions))
        .dev_tools(cfg!(debug_assertions))
        .general_autofill(false)
        .password_autosave(false)
        .pinch_zoom(false)
        .swipe_navigation(false)
        .zoom_control(false),
    )
    .build()
}

#[cfg(desktop)]
pub fn single_instance() -> TauriPlugin<Wry> {
  tauri_plugin_single_instance::init(|app, _, _| {
    let window = app.main_window();
    let _ = try {
      window.show()?;
      window.unminimize()?;
      window.set_focus()?;
    };
  })
}
