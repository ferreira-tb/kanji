use derive_more::{Deref, Display};
use diesel::backend::Backend;
use diesel::deserialize::{self as de, FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::serialize::{self as ser, IsNull, Output, ToSql};
use diesel::sql_types::Text;
use diesel::sqlite::Sqlite;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(
  FromSqlRow,
  AsExpression,
  Clone,
  Copy,
  Debug,
  Deref,
  Display,
  PartialEq,
  Eq,
  Hash,
  Deserialize,
  Serialize,
)]
#[diesel(sql_type = Text)]
pub struct QuizAnswerId(Uuid);

impl QuizAnswerId {
  pub fn new() -> Self {
    Self(Uuid::new_v4())
  }
}

impl Default for QuizAnswerId {
  fn default() -> Self {
    Self::new()
  }
}

impl FromSql<Text, Sqlite> for QuizAnswerId {
  fn from_sql(bytes: <Sqlite as Backend>::RawValue<'_>) -> de::Result<Self> {
    let value = <String as FromSql<Text, Sqlite>>::from_sql(bytes)?;
    let uuid = Uuid::try_parse(value.as_str())?;
    Ok(QuizAnswerId(uuid))
  }
}

impl ToSql<Text, Sqlite> for QuizAnswerId
where
  String: ToSql<Text, Sqlite>,
{
  fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> ser::Result {
    out.set_value(self.to_string());
    Ok(IsNull::No)
  }
}
