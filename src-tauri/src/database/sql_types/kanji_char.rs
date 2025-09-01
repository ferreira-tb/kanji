use crate::kanji::is_kanji;
use anyhow::{Result, anyhow};
use derive_more::{Deref, Display};
use diesel::backend::Backend;
use diesel::deserialize::{self as de, FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::serialize::{self as ser, IsNull, Output, ToSql};
use diesel::sql_types::Text;
use diesel::sqlite::Sqlite;
use serde::{Deserialize, Serialize};

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
pub struct KanjiChar(char);

impl KanjiChar {
  pub fn from_char(value: char) -> Option<Self> {
    is_kanji(value).then_some(Self(value))
  }
}

impl TryFrom<&str> for KanjiChar {
  type Error = anyhow::Error;

  fn try_from(value: &str) -> Result<Self> {
    value
      .chars()
      .next()
      .and_then(KanjiChar::from_char)
      .ok_or_else(|| anyhow!("\"{value}\" is not a valid kanji"))
  }
}

impl FromSql<Text, Sqlite> for KanjiChar {
  fn from_sql(bytes: <Sqlite as Backend>::RawValue<'_>) -> de::Result<Self> {
    let value = <String as FromSql<Text, Sqlite>>::from_sql(bytes)?;
    Ok(KanjiChar::try_from(value.as_str())?)
  }
}

impl ToSql<Text, Sqlite> for KanjiChar
where
  String: ToSql<Text, Sqlite>,
{
  fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> ser::Result {
    out.set_value(self.to_string());
    Ok(IsNull::No)
  }
}
