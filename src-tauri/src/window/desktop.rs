use crate::settings::Settings;
use anyhow::Result;
use tauri::{AppHandle, Manager, WebviewWindow, WebviewWindowBuilder, WindowEvent, Wry};

pub trait WindowExt: Manager<Wry> {
  fn main_window(&self) -> WebviewWindow<Wry> {
    self.get_webview_window("main").unwrap()
  }
}

impl<T: Manager<Wry>> WindowExt for T {}

pub fn open(app: &AppHandle) -> Result<()> {
  let window = WebviewWindowBuilder::new(app, "main", super::url())
    .title("Kanji")
    .initialization_script(super::script())
    .inner_size(1200.0, 800.0)
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
      && let Ok(settings) = Settings::get(&app)
      && settings.hide_on_close
    {
      api.prevent_close();
      app.main_window().hide().unwrap();
    }
  }
}
