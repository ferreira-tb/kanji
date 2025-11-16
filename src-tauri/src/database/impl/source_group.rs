use crate::database::model::source::Source;
use crate::database::model::source_group::{NewSourceGroup, SourceGroup, SourceGroupSource};
use crate::database::sql_types::{SourceGroupId, SourceId, Zoned};
use crate::database::{DatabaseHandle, schema};
use anyhow::Result;
use diesel::prelude::*;

impl DatabaseHandle {
  pub fn create_source_group(&self, new: &NewSourceGroup) -> Result<SourceGroupId> {
    use schema::source_group::dsl::*;
    diesel::insert_into(source_group)
      .values(new)
      .returning(id)
      .get_result(&mut *self.conn())
      .map_err(Into::into)
  }

  pub fn get_source_group(&self, group_id: SourceGroupId) -> Result<SourceGroup> {
    use schema::source_group::dsl::*;
    source_group
      .find(group_id)
      .select(SourceGroup::as_select())
      .first(&mut *self.conn())
      .map_err(Into::into)
  }

  pub fn get_source_group_ids(&self) -> Result<Vec<SourceGroupId>> {
    use schema::source_group::dsl::*;
    source_group
      .select(id)
      .load(&mut *self.conn())
      .map_err(Into::into)
  }

  pub fn get_source_group_source_ids(&self, group_id: SourceGroupId) -> Result<Vec<SourceId>> {
    use schema::source_group_source::dsl::*;
    source_group_source
      .filter(source_group_id.eq(group_id))
      .select(source_id)
      .load(&mut *self.conn())
      .map_err(Into::into)
  }

  pub fn get_source_group_sources(&self, group_id: SourceGroupId) -> Result<Vec<Source>> {
    use schema::source_group_source::dsl::*;
    source_group_source
      .filter(source_group_id.eq(group_id))
      .inner_join(schema::source::table)
      .select(Source::as_select())
      .load(&mut *self.conn())
      .map_err(Into::into)
  }

  pub fn get_source_groups(&self) -> Result<Vec<SourceGroup>> {
    use schema::source_group::dsl::*;
    source_group
      .select(SourceGroup::as_select())
      .order(name.asc())
      .load(&mut *self.conn())
      .map_err(Into::into)
  }

  pub fn remove_source_group(&self, group_id: SourceGroupId) -> Result<usize> {
    use schema::source_group::dsl::*;
    diesel::delete(source_group.find(group_id))
      .execute(&mut *self.conn())
      .map_err(Into::into)
  }

  pub fn rename_source_group(&self, group_id: SourceGroupId, new_name: &str) -> Result<()> {
    use schema::source_group::dsl::*;
    diesel::update(source_group.find(group_id))
      .set((name.eq(new_name), updated_at.eq(Zoned::now())))
      .execute(&mut *self.conn())
      .map(drop)
      .map_err(Into::into)
  }

  pub fn set_source_group_sources(
    &self,
    group_id: SourceGroupId,
    sources: &[SourceId],
  ) -> Result<()> {
    use schema::source_group_source::dsl::*;
    if sources.is_empty() {
      diesel::delete(source_group_source.filter(source_group_id.eq(group_id)))
        .execute(&mut *self.conn())?;
    } else {
      diesel::delete(
        source_group_source
          .filter(source_group_id.eq(group_id))
          .filter(source_id.ne_all(sources)),
      )
      .execute(&mut *self.conn())?;

      for source in sources {
        let new = SourceGroupSource {
          source_group_id: group_id,
          source_id: *source,
        };

        diesel::insert_into(source_group_source)
          .values(&new)
          .on_conflict_do_nothing()
          .execute(&mut *self.conn())?;
      }
    }

    Ok(())
  }
}
