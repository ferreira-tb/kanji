use crate::database::model::source::Source;
use crate::database::model::source_group::{NewSourceGroup, SourceGroup};
use crate::database::sql_types::{SourceGroupId, SourceId};
use crate::error::{CResult, Error};
use crate::manager::ManagerExt;
use tauri::AppHandle;

#[tauri::command]
pub async fn create_source_group(app: AppHandle, name: String) -> CResult<SourceGroupId> {
  let name = name.trim();
  if name.is_empty() {
    return Err(Error::from("source group name is empty"));
  }

  NewSourceGroup::builder()
    .name(name)
    .build()
    .create(&app)
    .map_err(Into::into)
}

#[tauri::command]
pub async fn get_source_group(app: AppHandle, id: SourceGroupId) -> CResult<SourceGroup> {
  app
    .database()
    .get_source_group(id)
    .map_err(Into::into)
}

#[tauri::command]
pub async fn get_source_group_ids(app: AppHandle) -> CResult<Vec<SourceGroupId>> {
  app
    .database()
    .get_source_group_ids()
    .map_err(Into::into)
}

#[tauri::command]
pub async fn get_source_group_source_ids(
  app: AppHandle,
  id: SourceGroupId,
) -> CResult<Vec<SourceId>> {
  app
    .database()
    .get_source_group_source_ids(id)
    .map_err(Into::into)
}

#[tauri::command]
pub async fn get_source_group_sources(app: AppHandle, id: SourceGroupId) -> CResult<Vec<Source>> {
  app
    .database()
    .get_source_group_sources(id)
    .map_err(Into::into)
}

#[tauri::command]
pub async fn get_source_groups(app: AppHandle) -> CResult<Vec<SourceGroup>> {
  app
    .database()
    .get_source_groups()
    .map_err(Into::into)
}

#[tauri::command]
pub async fn remove_source_group(app: AppHandle, id: SourceGroupId) -> CResult<usize> {
  app
    .database()
    .remove_source_group(id)
    .map_err(Into::into)
}

#[tauri::command]
pub async fn rename_source_group(app: AppHandle, id: SourceGroupId, name: String) -> CResult<()> {
  app
    .database()
    .rename_source_group(id, &name)
    .map_err(Into::into)
}

#[tauri::command]
pub async fn set_source_group_sources(
  app: AppHandle,
  id: SourceGroupId,
  sources: Vec<SourceId>,
) -> CResult<()> {
  app
    .database()
    .set_source_group_sources(id, &sources)
    .map_err(Into::into)
}
