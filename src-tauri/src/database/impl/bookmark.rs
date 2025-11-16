use crate::database::model::bookmark::{Bookmark, NewBookmark};
use crate::database::sql_types::BookmarkId;
use crate::database::{DatabaseHandle, schema};
use anyhow::Result;
use diesel::prelude::*;

impl DatabaseHandle {
  pub fn create_bookmark(&self, new: &NewBookmark) -> Result<BookmarkId> {
    use schema::bookmark::dsl::*;
    diesel::insert_into(bookmark)
      .values(new)
      .returning(id)
      .get_result(&mut *self.conn())
      .map_err(Into::into)
  }

  pub fn get_bookmark_id(&self, text: &str) -> Result<Option<BookmarkId>> {
    use schema::bookmark::dsl::*;
    bookmark
      .filter(snippet.eq(text))
      .select(id)
      .first::<BookmarkId>(&mut *self.conn())
      .optional()
      .map_err(Into::into)
  }

  pub fn get_bookmarks(&self) -> Result<Vec<Bookmark>> {
    use schema::bookmark::dsl::*;
    bookmark
      .select(Bookmark::as_select())
      .load(&mut *self.conn())
      .map_err(Into::into)
  }

  pub fn remove_bookmark(&self, bookmark_id: BookmarkId) -> Result<usize> {
    use schema::bookmark::dsl::*;
    diesel::delete(bookmark.find(bookmark_id))
      .execute(&mut *self.conn())
      .map_err(Into::into)
  }
}
