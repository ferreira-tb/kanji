use anyhow::Result;
use serde::Deserialize;
use tauri::AppHandle;
use tauri_plugin_pinia::ManagerExt as _;

#[derive(Clone, Debug, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Settings {
  pub hide_on_close: bool,
  pub snippet_limit: usize,
  pub snippet_min_len: usize,
  pub snippet_kanji_threshold: f64,
  pub shuffle_snippets: bool,
  pub set_file_name: Box<str>,
  pub set_size: usize,
}

impl Settings {
  pub const DEFAULT_HIDE_ON_CLOSE: bool = false;
  pub const DEFAULT_SNIPPET_LIMIT: usize = 1000;
  pub const DEFAULT_SNIPPET_MIN_LEN: usize = 5;
  pub const DEFAULT_SNIPPET_KANJI_THRESHOLD: f64 = 0.2;
  pub const DEFAULT_SHUFFLE_SNIPPETS: bool = true;
  pub const DEFAULT_SET_SIZE: usize = 50;

  pub fn get(app: &AppHandle) -> Result<Self> {
    app
      .pinia()
      .state_or_default("settings")
      .map_err(Into::into)
  }
}

impl Default for Settings {
  fn default() -> Self {
    Self {
      hide_on_close: Self::DEFAULT_HIDE_ON_CLOSE,
      snippet_limit: Self::DEFAULT_SNIPPET_LIMIT,
      snippet_min_len: Self::DEFAULT_SNIPPET_MIN_LEN,
      snippet_kanji_threshold: Self::DEFAULT_SNIPPET_KANJI_THRESHOLD,
      shuffle_snippets: Self::DEFAULT_SHUFFLE_SNIPPETS,
      set_file_name: Box::from("Kanji Set.txt"),
      set_size: Self::DEFAULT_SET_SIZE,
    }
  }
}
