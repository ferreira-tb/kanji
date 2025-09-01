use crate::database::sql_types::KanjiChar;
use crate::kanji::{KanjiStats, search as search_kanji};
use crate::settings::Settings;
use anyhow::Result;
use derive_more::Deref;
use itertools::Itertools;
use std::path::Path as StdPath;
use tauri::AppHandle;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

#[derive(Debug, Deref)]
pub struct KanjiSet(Box<[KanjiSetChunk]>);

impl KanjiSet {
  pub async fn load(app: AppHandle) -> Result<Self> {
    let settings = Settings::get(&app)?;
    let mut kanjis = search_kanji(app).await?;
    kanjis.sort_by_key(KanjiStats::seen);

    let mut sets = Vec::new();
    for chunk in &kanjis
      .iter()
      .map(KanjiStats::character)
      .rev()
      .chunks(settings.set_size)
    {
      sets.push(chunk.collect());
    }

    Ok(Self(sets.into_boxed_slice()))
  }

  pub async fn write(self, app: AppHandle, folder: &StdPath) -> Result<()> {
    let settings = Settings::get(&app)?;
    let sets = KanjiSet::load(app).await?;
    let path = folder.join(settings.set_file_name.as_ref());
    let mut file = File::create(path).await?;

    for chunk in &*sets {
      let chunk = chunk.iter().join("");
      let bytes = Vec::from_iter(chunk.bytes());
      file.write(&bytes).await?;
      file.write(&[b'\n']).await?;
    }

    file.flush().await?;

    Ok(())
  }
}

#[derive(Debug, Deref)]
pub struct KanjiSetChunk(Box<[KanjiChar]>);

impl FromIterator<KanjiChar> for KanjiSetChunk {
  fn from_iter<T>(iter: T) -> Self
  where
    T: IntoIterator<Item = KanjiChar>,
  {
    let chunk = iter
      .into_iter()
      .collect_vec()
      .into_boxed_slice();

    Self(chunk)
  }
}
