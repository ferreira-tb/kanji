use crate::database::model::kanji::NewKanji;
use crate::database::sql_types::KanjiChar;
use crate::database::{DatabaseHandle, schema};
use anyhow::Result;
use diesel::prelude::*;

impl DatabaseHandle {
  pub fn create_kanji(&self, new: &NewKanji) -> Result<()> {
    use schema::kanji::dsl::*;
    diesel::insert_into(kanji)
      .values(new)
      .on_conflict(id)
      .do_nothing()
      .execute(&mut *self.conn())
      .map(drop)
      .map_err(Into::into)
  }

  pub fn get_kanji_chars(&self) -> Result<Vec<KanjiChar>> {
    use schema::kanji::dsl::*;
    kanji
      .select(id)
      .load(&mut *self.conn())
      .map_err(Into::into)
  }
}
