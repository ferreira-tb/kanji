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

  pub fn has_kanji(&self, kanji_char: KanjiChar) -> Result<bool> {
    use schema::kanji::dsl::*;
    kanji
      .find(kanji_char)
      .select(id)
      .first::<KanjiChar>(&mut *self.conn())
      .optional()
      .map(|it| it.is_some())
      .map_err(Into::into)
  }
}
