use crate::database::sql_types::KanjiChar;
use crate::kanji::{KanjiStats, search as search_kanji};
use crate::settings::Settings;
use anyhow::Result;
use derive_more::Deref;
use itertools::Itertools;
use serde::Serialize;
use std::path::Path as StdPath;
use tauri::AppHandle;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

#[derive(Debug, Deref, Serialize)]
pub struct KanjiSet(Box<[KanjiSetChunk]>);

impl KanjiSet {
  pub async fn load(app: AppHandle) -> Result<Self> {
    let settings = Settings::get(&app)?;
    let mut kanjis = search_kanji(app).await?;
    kanjis.sort_by_key(KanjiStats::seen);

    let mut sets = Vec::new();
    let chunks = kanjis
      .iter()
      .map(KanjiStats::character)
      .rev()
      .chunks(settings.set_size);

    for (id, chunk) in (1u32..).zip(&chunks) {
      let id = KanjiSetChunkId(id);
      let kanjis = chunk
        .into_iter()
        .collect_vec()
        .into_boxed_slice();

      sets.push(KanjiSetChunk { id, kanjis });
    }

    Ok(Self(sets.into_boxed_slice()))
  }

  pub async fn export(self, app: AppHandle, folder: &StdPath) -> Result<()> {
    let settings = Settings::get(&app)?;
    let sets = KanjiSet::load(app).await?;
    let path = folder.join(settings.set_file_name.as_ref());
    let mut file = File::create(path).await?;

    for chunk in &sets.0 {
      let chunk = chunk.kanjis.iter().join("");
      let bytes = chunk.bytes().collect_vec();
      file.write_all(&bytes).await?;
      file.write_all(b"\n").await?;
    }

    file.flush().await?;

    Ok(())
  }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KanjiSetChunk {
  id: KanjiSetChunkId,
  kanjis: Box<[KanjiChar]>,
}

#[derive(Debug, Serialize)]
pub struct KanjiSetChunkId(u32);
