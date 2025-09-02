mod migration;
pub mod model;
pub mod schema;
pub mod sql_types;

use crate::database::model::kanji::NewKanji;
use crate::database::model::quiz_answer::NewQuizAnswer;
use crate::database::model::source::{NewSource, Source};
use crate::database::sql_types::{KanjiChar, SourceId, Zoned};
use anyhow::Result;
use diesel::Connection;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use std::sync::{Arc, Mutex, MutexGuard};

#[cfg(debug_assertions)]
const URL: &str = env!("KANJI_DATABASE_URL_DEBUG");
#[cfg(not(debug_assertions))]
const URL: &str = env!("KANJI_DATABASE_URL");

#[derive(Clone)]
pub struct DatabaseHandle(Arc<Mutex<SqliteConnection>>);

impl DatabaseHandle {
  pub fn new() -> Result<Self> {
    let mut conn = SqliteConnection::establish(URL)?;
    migration::run_pending_migrations(&mut conn);
    Ok(Self(Arc::new(Mutex::new(conn))))
  }

  pub fn conn(&self) -> MutexGuard<'_, SqliteConnection> {
    self
      .0
      .lock()
      .expect("connection mutex is poisoned")
  }

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

  pub fn create_source(&self, new: &NewSource) -> Result<SourceId> {
    use schema::source::dsl::*;
    diesel::insert_into(source)
      .values(new)
      .returning(id)
      .get_result(&mut *self.conn())
      .map_err(Into::into)
  }

  pub fn get_source(&self, source_id: SourceId) -> Result<Source> {
    use schema::source::dsl::*;
    source
      .find(source_id)
      .select(Source::as_select())
      .first(&mut *self.conn())
      .map_err(Into::into)
  }

  pub fn get_sources(&self) -> Result<Vec<Source>> {
    use schema::source::dsl::*;
    source
      .select(Source::as_select())
      .load(&mut *self.conn())
      .map_err(Into::into)
  }

  pub fn rename_source(&self, source_id: SourceId, new_name: &str) -> Result<()> {
    use schema::source::dsl::*;
    diesel::update(source.find(source_id))
      .set((name.eq(new_name), updated_at.eq(Zoned::now())))
      .execute(&mut *self.conn())
      .map(drop)
      .map_err(Into::into)
  }

  pub fn create_quiz_answer(&self, new: &NewQuizAnswer) -> Result<()> {
    use schema::quiz_answer::dsl::*;
    diesel::insert_into(quiz_answer)
      .values(new)
      .execute(&mut *self.conn())
      .map(drop)
      .map_err(Into::into)
  }

  pub fn count_quizzes(&self, kanji_char: KanjiChar) -> Result<u64> {
    use schema::quiz_answer::dsl::*;
    quiz_answer
      .filter(question.eq(kanji_char))
      .count()
      .get_result::<i64>(&mut *self.conn())
      .map(u64::try_from)?
      .map_err(Into::into)
  }

  pub fn count_quizzes_in(&self, kanji_chars: &[KanjiChar]) -> Result<u64> {
    use schema::quiz_answer::dsl::*;
    quiz_answer
      .filter(question.eq_any(kanji_chars))
      .count()
      .get_result::<i64>(&mut *self.conn())
      .map(u64::try_from)?
      .map_err(Into::into)
  }

  pub fn count_correct_quiz_answers(&self, kanji_char: KanjiChar) -> Result<u64> {
    use schema::quiz_answer::dsl::*;
    quiz_answer
      .filter(question.eq(kanji_char))
      .filter(answer.eq(kanji_char))
      .count()
      .get_result::<i64>(&mut *self.conn())
      .map(u64::try_from)?
      .map_err(Into::into)
  }

  pub fn count_correct_quiz_answers_in(&self, kanji_chars: &[KanjiChar]) -> Result<u64> {
    let mut count = 0u64;
    for kanji_char in kanji_chars {
      let correct = self.count_correct_quiz_answers(*kanji_char)?;
      count = count.saturating_add(correct)
    }

    Ok(count)
  }
}
