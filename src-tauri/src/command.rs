use crate::database::model::bookmark::Bookmark;
use crate::database::model::quiz_answer::{NewQuizAnswer, QuizAnswer};
use crate::database::model::source::{NewSource, Source};
use crate::database::sql_types::{
  BookmarkId,
  KanjiChar,
  Path,
  QuizAnswerId,
  SourceId,
  SourceWeight,
};
use crate::error::{CResult, Error};
use crate::kanji::{self, KanjiStats};
use crate::manager::ManagerExt;
use crate::quiz::Quiz;
use crate::set::KanjiSet;
use crate::snippet::{self, Snippet};
use crate::tray;
use itertools::Itertools;
use std::net::SocketAddrV4;
use std::path::PathBuf;
use std::process::Stdio;
use tauri::async_runtime::spawn_blocking;
use tauri::{AppHandle, WebviewWindow};
use tauri_plugin_dialog::DialogExt;
use tauri_plugin_fs::{FilePath, FsExt};
use tokio::process::Command;
use tokio::sync::oneshot;
use windows::Win32::System::Threading::{CREATE_NEW_PROCESS_GROUP, CREATE_NO_WINDOW};

#[tauri::command]
pub async fn create_bookmark(app: AppHandle, snippet: Snippet) -> CResult<BookmarkId> {
  snippet
    .create_bookmark(&app)
    .map_err(Into::into)
}

#[tauri::command]
pub async fn create_quiz(app: AppHandle, kanjis: Vec<KanjiChar>) -> CResult<Quiz> {
  Quiz::new(app, kanjis)
    .await
    .map_err(Into::into)
}

#[tauri::command]
pub async fn create_quiz_answer(
  app: AppHandle,
  question: KanjiChar,
  answer: KanjiChar,
) -> CResult<QuizAnswerId> {
  NewQuizAnswer::builder()
    .question(question)
    .answer(answer)
    .build()
    .create(&app)
    .map_err(Into::into)
}

#[tauri::command]
pub async fn create_random_quiz(app: AppHandle) -> CResult<Quiz> {
  Quiz::random(app).await.map_err(Into::into)
}

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
  if let Some(folder) = pick_folders(app.clone()).await?.first() {
    KanjiSet::load(app.clone())
      .await?
      .export(app, folder)
      .await?;
  }

  Ok(())
}

#[tauri::command]
pub async fn get_bookmarks(app: AppHandle) -> CResult<Vec<Bookmark>> {
  app
    .database()
    .get_bookmarks()
    .map_err(Into::into)
}

#[tauri::command]
pub async fn get_quiz_answers(app: AppHandle) -> CResult<Vec<QuizAnswer>> {
  let task = spawn_blocking(move || {
    app
      .database()
      .get_quiz_answers()
      .map_err(Into::into)
  });

  task.await?
}

#[tauri::command]
pub async fn get_server_addr(app: AppHandle) -> SocketAddrV4 {
  app.server().addr()
}

#[tauri::command]
pub async fn get_set(app: AppHandle) -> CResult<KanjiSet> {
  KanjiSet::load(app).await.map_err(Into::into)
}

#[tauri::command]
pub async fn get_sources(app: AppHandle) -> CResult<Vec<Source>> {
  let mut sources = app.database().get_sources()?;
  sources.sort_by(|a, b| {
    if let Some(dir_a) = a.path.file_stem()
      && let Some(dir_b) = b.path.file_stem()
      && let Some(dir_a) = dir_a.to_str()
      && let Some(dir_b) = dir_b.to_str()
    {
      dir_a.cmp(dir_b)
    } else {
      a.name.cmp(&b.name)
    }
  });

  Ok(sources)
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
pub async fn rename_source(app: AppHandle, id: SourceId, name: String) -> CResult<()> {
  app
    .database()
    .rename_source(id, &name)
    .map_err(Into::into)
}
#[tauri::command]
pub async fn remove_bookmark(app: AppHandle, id: BookmarkId) -> CResult<usize> {
  app
    .database()
    .remove_bookmark(id)
    .map_err(Into::into)
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
pub async fn set_source_weight(app: AppHandle, id: SourceId, weight: SourceWeight) -> CResult<()> {
  app
    .database()
    .set_source_weight(id, weight)
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

#[tauri::command]
pub async fn toggle_source(app: AppHandle, id: SourceId, enabled: bool) -> CResult<()> {
  app
    .database()
    .toggle_source(id, enabled)
    .map_err(Into::into)
}
