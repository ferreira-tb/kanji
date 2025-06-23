use anyhow::Result;
use tauri_plugin_pinia::ManagerExt as _;

use tauri::{
  AppHandle,
  Manager,
  WebviewUrl,
  WebviewWindow,
  WebviewWindowBuilder,
  WindowEvent,
  Wry,
};

pub trait WindowExt: Manager<Wry> {
  fn main_window(&self) -> WebviewWindow<Wry> {
    self.get_webview_window("main").unwrap()
  }
}

impl<T: Manager<Wry>> WindowExt for T {}

pub fn open(app: &AppHandle) -> Result<()> {
  let url = WebviewUrl::App("index.html".into());
  let window = WebviewWindowBuilder::new(app, "main", url)
    .title("Kanji Frequency")
    .inner_size(1200.0, 800.0)
    .min_inner_size(800.0, 600.0)
    .resizable(true)
    .maximizable(true)
    .minimizable(true)
    .visible(false)
    .prevent_overflow()
    .build()?;

  window.on_window_event(on_window_event(app));

  Ok(())
}

fn on_window_event(app: &AppHandle) -> impl Fn(&WindowEvent) + use<> {
  let app = app.clone();
  move |event| {
    if let WindowEvent::CloseRequested { api, .. } = event
      && app
        .pinia()
        .try_get_or_default("settings", "hideOnClose")
    {
      api.prevent_close();
      app.main_window().hide().unwrap();
    }
  }
}
