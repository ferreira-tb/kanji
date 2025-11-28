use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use strum::AsRefStr;
use tauri::AppHandle;
use tauri_plugin_pinia::ManagerExt as _;

pub const DEFAULT_EDITOR: Editor = Editor::Code;
pub const DEFAULT_HIDE_ON_CLOSE: bool = false;
pub const DEFAULT_IGNORE_SOURCE_WEIGHT: bool = false;
pub const DEFAULT_SET_CHUNK_SIZE: usize = 25;
pub const DEFAULT_SET_FILE_NAME: &str = "Kanji Set.txt";
pub const DEFAULT_SHUFFLE_SNIPPETS: bool = true;
pub const DEFAULT_SNIPPET_LIMIT: usize = 1000;
pub const DEFAULT_SNIPPET_MIN_LEN: usize = 5;

#[derive(Clone, Copy, Debug, Default, AsRefStr, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
pub enum Editor {
  #[default]
  Code,
  CodeInsiders,
  Zed,
}

fn get_or<T>(app: &AppHandle, key: &str, default: T) -> T
where
  T: DeserializeOwned,
{
  app.pinia().get_or("settings", key, default)
}

fn get_or_default<T>(app: &AppHandle, key: &str) -> T
where
  T: Default + DeserializeOwned,
{
  app.pinia().get_or_default("settings", key)
}

fn get_or_else<T, F>(app: &AppHandle, key: &str, f: F) -> T
where
  T: DeserializeOwned,
  F: FnOnce() -> T,
{
  app.pinia().get_or_else("settings", key, f)
}

pub fn editor(app: &AppHandle) -> Editor {
  get_or(app, "editor", DEFAULT_EDITOR)
}

pub fn forbidden_words(app: &AppHandle) -> Vec<String> {
  get_or_default::<String>(app, "forbiddenWords")
    .split('\n')
    .map(str::trim)
    .filter(|word| !word.is_empty())
    .map(ToOwned::to_owned)
    .collect()
}

pub fn hide_on_close(app: &AppHandle) -> bool {
  get_or(app, "hideOnClose", DEFAULT_HIDE_ON_CLOSE)
}

pub fn ignore_source_weight(app: &AppHandle) -> bool {
  get_or(app, "ignoreSourceWeight", DEFAULT_IGNORE_SOURCE_WEIGHT)
}

pub fn set_chunk_size(app: &AppHandle) -> usize {
  get_or(app, "setChunkSize", DEFAULT_SET_CHUNK_SIZE)
}

pub fn set_file_name(app: &AppHandle) -> String {
  get_or_else(app, "setFileName", || DEFAULT_SET_FILE_NAME.to_owned())
}

pub fn shuffle_snippets(app: &AppHandle) -> bool {
  get_or(app, "shuffleSnippets", DEFAULT_SHUFFLE_SNIPPETS)
}

pub fn snippet_limit(app: &AppHandle) -> usize {
  get_or(app, "snippetLimit", DEFAULT_SNIPPET_LIMIT)
}

pub fn snippet_min_len(app: &AppHandle) -> usize {
  get_or(app, "snippetMinLen", DEFAULT_SNIPPET_MIN_LEN)
}
