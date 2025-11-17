use crate::database::model::source::{NewSource, Source};
use crate::database::sql_types::{Path, SourceId, SourceWeight};
use crate::error::{CResult, Error};
use crate::manager::ManagerExt;
use tauri::AppHandle;

#[tauri::command]
pub async fn create_source(app: AppHandle, source: Path) -> CResult<SourceId> {
  let name: Option<String> = try { source.file_stem()?.to_str()?.to_owned() };
  let Some(name) = name else {
    return Err(Error::from(format!("Invalid source: {source}")));
  };

  NewSource::builder(source)
    .name(name.as_str())
    .build()
    .create(&app)
    .map_err(Into::into)
}

#[tauri::command]
pub async fn get_source(app: AppHandle, id: SourceId) -> CResult<Source> {
  app
    .database()
    .get_source(id)
    .map_err(Into::into)
}

#[tauri::command]
pub async fn get_sources(app: AppHandle) -> CResult<Vec<Source>> {
  app
    .database()
    .get_sources()
    .map_err(Into::into)
}

#[tauri::command]
pub async fn remove_source(app: AppHandle, id: SourceId) -> CResult<usize> {
  app
    .database()
    .remove_source(id)
    .map_err(Into::into)
}

#[tauri::command]
pub async fn rename_source(app: AppHandle, id: SourceId, name: String) -> CResult<()> {
  app
    .database()
    .rename_source(id, &name)
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
pub async fn toggle_source(app: AppHandle, id: SourceId, enabled: bool) -> CResult<()> {
  app
    .database()
    .toggle_source(id, enabled)
    .map_err(Into::into)
}
