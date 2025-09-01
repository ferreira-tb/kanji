use crate::database::model::source::NewSource;
use crate::database::sql_types::{KanjiChar, Path, SourceId};
use crate::error::{CResult, Error};
use crate::kanji::{self, KanjiStats};
use crate::settings::Settings;
use crate::snippet::{self, Snippet};
use crate::tray;
use itertools::Itertools;
use std::path::PathBuf;
use std::process::Stdio;
use tauri::{AppHandle, WebviewWindow};
use tauri_plugin_dialog::DialogExt;
use tauri_plugin_fs::{FilePath, FsExt};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::process::Command;
use tokio::sync::oneshot;
use windows::Win32::System::Threading::{CREATE_NEW_PROCESS_GROUP, CREATE_NO_WINDOW};

#[tauri::command]
pub async fn create_source(app: AppHandle, source: Path) -> CResult<SourceId> {
  let name: Option<String> = try { source.file_stem()?.to_str()?.to_owned() };
  let Some(name) = name else {
    return Err(Error::from(format!("invalid source: {source}")));
  };

  NewSource::builder(source)
    .name(name.as_str())
    .build()
    .create(&app)
    .map_err(Into::into)
}

#[tauri::command]
pub async fn create_tray_icon(app: AppHandle) -> CResult<()> {
  let handle = app.clone();
  handle
    .run_on_main_thread(move || tray::create(&app).unwrap())
    .map_err(Into::into)
}

#[tauri::command]
pub async fn export_set(app: AppHandle) -> CResult<()> {
  if let Some(folder) = pick_folders(app.clone()).await?.get(0) {
    let mut kanji = search_kanji(app.clone()).await?;
    kanji.sort_by_key(KanjiStats::seen);

    let chunk_size = 50;
    let capacity = kanji
      .len()
      .saturating_mul(2)
      .saturating_add(chunk_size);

    let mut set = Vec::with_capacity(capacity);

    for mut chunk in &kanji
      .iter()
      .map(KanjiStats::character)
      .rev()
      .chunks(chunk_size)
    {
      let chunk = chunk.join("");
      set.extend(chunk.bytes());
      set.push(b'\n');
    }

    let settings = Settings::get(&app)?;
    let path = folder.join(settings.set_file_name.as_ref());
    let mut file = File::create(path).await?;
    file.write_all(&set).await?;
    file.flush().await?;
  }

  Ok(())
}

#[tauri::command]
pub async fn open(path: PathBuf, line: u32) -> CResult<()> {
  let path = format!("{}:{}", path.to_string_lossy(), line);
  Command::new("pwsh")
    .args(["-Command", "code", "--goto", path.as_str()])
    .creation_flags(CREATE_NEW_PROCESS_GROUP.0 | CREATE_NO_WINDOW.0)
    .stdin(Stdio::null())
    .stdout(Stdio::null())
    .stderr(Stdio::null())
    .status()
    .await?;

  Ok(())
}

#[tauri::command]
pub async fn pick_folders(app: AppHandle) -> CResult<Vec<PathBuf>> {
  let (tx, rx) = oneshot::channel();
  app
    .dialog()
    .file()
    .pick_folders(move |response| {
      let folders = response
        .unwrap_or_default()
        .into_iter()
        .map(FilePath::into_path)
        .process_results(|it| it.collect_vec());

      let _ = tx.send(folders);
    });

  let folders = rx.await??;
  for folder in &folders {
    let scope = app.fs_scope();
    if !scope.is_allowed(folder) {
      let _ = scope.allow_directory(folder, true);
    }
  }

  Ok(folders)
}

#[tauri::command]
pub async fn search_kanji(app: AppHandle) -> CResult<Vec<KanjiStats>> {
  kanji::search(app).await.map_err(Into::into)
}

#[tauri::command]
pub async fn search_snippets(
  app: AppHandle,
  kanji: KanjiChar,
  source: Option<SourceId>,
) -> CResult<Vec<Snippet>> {
  snippet::search(app, kanji, source)
    .await
    .map_err(Into::into)
}

#[tauri::command]
pub async fn show_window(window: WebviewWindow) -> CResult<()> {
  window
    .show()
    .and_then(|()| window.unminimize())
    .and_then(|()| window.set_focus())
    .map_err(Into::into)
}
