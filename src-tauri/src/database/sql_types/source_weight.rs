use derive_more::Deref;
use diesel::backend::Backend;
use diesel::deserialize::{self as de, FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::serialize::{self as ser, IsNull, Output, ToSql};
use diesel::sql_types::Integer;
use diesel::sqlite::Sqlite;
use serde::{Deserialize, Serialize};
use std::num::NonZeroU8;

#[derive(FromSqlRow, AsExpression, Debug, Deref, Clone, Copy, Deserialize, Serialize)]
#[diesel(sql_type = Integer)]
pub struct SourceWeight(NonZeroU8);

impl SourceWeight {
  const MIN: u8 = 1;
  const MAX: u8 = 5;

  pub fn new(weight: u8) -> Self {
    let weight = weight.clamp(Self::MIN, Self::MAX);
    Self(unsafe { NonZeroU8::new_unchecked(weight) })
  }
}

impl From<u8> for SourceWeight {
  fn from(weight: u8) -> Self {
    Self::new(weight)
  }
}

impl From<SourceWeight> for f64 {
  fn from(weight: SourceWeight) -> Self {
    f64::from(weight.0.get())
  }
}

impl FromSql<Integer, Sqlite> for SourceWeight
where
  i32: FromSql<Integer, Sqlite>,
{
  fn from_sql(bytes: <Sqlite as Backend>::RawValue<'_>) -> de::Result<Self> {
    let value: u8 = i32::from_sql(bytes)?.try_into()?;
    NonZeroU8::try_from(value)
      .map(SourceWeight)
      .map_err(Into::into)
  }
}

impl ToSql<Integer, Sqlite> for SourceWeight
where
  i32: ToSql<Integer, Sqlite>,
{
  fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> ser::Result {
    let weight = self.0.get().clamp(Self::MIN, Self::MAX);
    out.set_value(i32::from(weight));
    Ok(IsNull::No)
  }
}
