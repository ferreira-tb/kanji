use anyhow::Result;
use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindow, WebviewWindowBuilder, Window, Wry};

pub trait WindowExt: Manager<Wry> {
  fn main_window(&self) -> WebviewWindow<Wry> {
    self.get_webview_window("main").unwrap()
  }
}

impl WindowExt for AppHandle<Wry> {}
impl WindowExt for WebviewWindow<Wry> {}
impl WindowExt for Window<Wry> {}

pub fn open(app: &AppHandle) -> Result<()> {
  let url = WebviewUrl::App("index.html".into());
  WebviewWindowBuilder::new(app, "main", url)
    .title("Kanji Frequency")
    .inner_size(800.0, 600.0)
    .min_inner_size(800.0, 600.0)
    .resizable(true)
    .maximizable(true)
    .minimizable(true)
    .visible(false)
    .prevent_overflow()
    .build()?;

  Ok(())
}
