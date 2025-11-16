use crate::database::model::quiz_answer::{NewQuizAnswer, QuizAnswer};
use crate::database::sql_types::{KanjiChar, QuizAnswerId, SourceId};
use crate::database::{DatabaseHandle, schema};
use anyhow::Result;
use diesel::prelude::*;

impl DatabaseHandle {
  pub fn create_quiz_answer(&self, new: &NewQuizAnswer) -> Result<QuizAnswerId> {
    use schema::quiz_answer::dsl::*;
    diesel::insert_into(quiz_answer)
      .values(new)
      .returning(id)
      .get_result(&mut *self.conn())
      .map_err(Into::into)
  }

  pub fn get_quiz_answers(&self) -> Result<Vec<QuizAnswer>> {
    use schema::quiz_answer::dsl::*;
    quiz_answer
      .select(QuizAnswer::as_select())
      .order(id.desc())
      .load(&mut *self.conn())
      .map_err(Into::into)
  }

  pub fn count_quizzes(&self, kanji: KanjiChar) -> Result<u64> {
    use schema::quiz_answer::dsl::*;
    quiz_answer
      .filter(question.eq(kanji))
      .count()
      .get_result::<i64>(&mut *self.conn())
      .map(u64::try_from)?
      .map_err(Into::into)
  }

  pub fn count_quizzes_in(&self, kanjis: &[KanjiChar]) -> Result<u64> {
    use schema::quiz_answer::dsl::*;
    quiz_answer
      .filter(question.eq_any(kanjis))
      .count()
      .get_result::<i64>(&mut *self.conn())
      .map(u64::try_from)?
      .map_err(Into::into)
  }

  pub fn count_quizzes_with_source(&self, source: SourceId) -> Result<u64> {
    use schema::quiz_answer::dsl::*;
    quiz_answer
      .filter(source_id.eq(source))
      .count()
      .get_result::<i64>(&mut *self.conn())
      .map(u64::try_from)?
      .map_err(Into::into)
  }

  pub fn count_correct_quizzes(&self, kanji: KanjiChar) -> Result<u64> {
    use schema::quiz_answer::dsl::*;
    quiz_answer
      .filter(question.eq(kanji))
      .filter(answer.eq(kanji))
      .count()
      .get_result::<i64>(&mut *self.conn())
      .map(u64::try_from)?
      .map_err(Into::into)
  }

  pub fn count_correct_quizzes_in(&self, kanjis: &[KanjiChar]) -> Result<u64> {
    let mut count = 0u64;
    for kanji in kanjis {
      let correct = self.count_correct_quizzes(*kanji)?;
      count = count.saturating_add(correct);
    }

    Ok(count)
  }

  pub fn count_correct_quizzes_with_source(&self, source: SourceId) -> Result<u64> {
    use schema::quiz_answer::dsl::*;
    quiz_answer
      .filter(question.eq(answer))
      .filter(source_id.eq(source))
      .count()
      .get_result::<i64>(&mut *self.conn())
      .map(u64::try_from)?
      .map_err(Into::into)
  }
}
