use crate::database::model::source::{NewSource, Source};
use crate::database::sql_types::{SourceId, SourceWeight, Zoned};
use crate::database::{DatabaseHandle, schema};
use anyhow::Result;
use diesel::prelude::*;

impl DatabaseHandle {
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

  pub fn get_source_ids(&self) -> Result<Vec<SourceId>> {
    use schema::source::dsl::*;
    source
      .select(id)
      .load(&mut *self.conn())
      .map_err(Into::into)
  }

  pub fn get_sources(&self) -> Result<Vec<Source>> {
    use schema::source::dsl::*;
    source
      .select(Source::as_select())
      .load(&mut *self.conn())
      .map_err(Into::into)
  }

  pub fn get_sources_by(&self, ids: &[SourceId]) -> Result<Vec<Source>> {
    use schema::source::dsl::*;
    source
      .select(Source::as_select())
      .filter(id.eq_any(ids))
      .load(&mut *self.conn())
      .map_err(Into::into)
  }

  pub fn get_enabled_sources(&self) -> Result<Vec<Source>> {
    use schema::source::dsl::*;
    source
      .filter(enabled.eq(true))
      .select(Source::as_select())
      .load(&mut *self.conn())
      .map_err(Into::into)
  }

  pub fn remove_source(&self, source_id: SourceId) -> Result<usize> {
    use schema::source::dsl::*;
    diesel::delete(source.find(source_id))
      .execute(&mut *self.conn())
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

  pub fn set_source_weight(&self, source_id: SourceId, new_weight: SourceWeight) -> Result<()> {
    use schema::source::dsl::*;
    diesel::update(source.find(source_id))
      .set((weight.eq(new_weight), updated_at.eq(Zoned::now())))
      .execute(&mut *self.conn())
      .map(drop)
      .map_err(Into::into)
  }

  pub fn toggle_source(&self, source_id: SourceId, is_enabled: bool) -> Result<()> {
    use schema::source::dsl::*;
    diesel::update(source.find(source_id))
      .set((enabled.eq(is_enabled), updated_at.eq(Zoned::now())))
      .execute(&mut *self.conn())
      .map(drop)
      .map_err(Into::into)
  }
}
