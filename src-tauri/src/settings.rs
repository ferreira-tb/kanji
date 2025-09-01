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
  pub shuffle_snippets: bool,
  pub set_file_name: Box<str>,
}

impl Settings {
  pub fn get(app: &AppHandle) -> Result<Self> {
    app
      .pinia()
      .try_state_or_default("settings")
      .map_err(Into::into)
  }
}

impl Default for Settings {
  fn default() -> Self {
    Self {
      hide_on_close: false,
      snippet_limit: 1000,
      snippet_min_len: 5,
      shuffle_snippets: true,
      set_file_name: Box::from("Kanji Set.txt"),
    }
  }
}
