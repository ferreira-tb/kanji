use derive_more::Deref;
use diesel::backend::Backend;
use diesel::deserialize::{self as de, FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::serialize::{self as ser, IsNull, Output, ToSql};
use diesel::sql_types::Integer;
use diesel::sqlite::Sqlite;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::num::NonZeroU32;

#[derive(
  FromSqlRow,
  AsExpression,
  Debug,
  Deref,
  Clone,
  Copy,
  PartialEq,
  Eq,
  PartialOrd,
  Ord,
  Hash,
  Deserialize,
  Serialize,
)]
#[diesel(sql_type = Integer)]
pub struct SourceId(NonZeroU32);

impl FromSql<Integer, Sqlite> for SourceId
where
  i32: FromSql<Integer, Sqlite>,
{
  fn from_sql(bytes: <Sqlite as Backend>::RawValue<'_>) -> de::Result<Self> {
    let value: u32 = i32::from_sql(bytes)?.try_into()?;
    NonZeroU32::try_from(value)
      .map(SourceId)
      .map_err(Into::into)
  }
}

impl ToSql<Integer, Sqlite> for SourceId
where
  i32: ToSql<Integer, Sqlite>,
{
  fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> ser::Result {
    out.set_value(i32::try_from(self.0.get())?);
    Ok(IsNull::No)
  }
}

impl fmt::Display for SourceId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}
