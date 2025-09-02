use derive_more::{Deref, Display};
use diesel::backend::Backend;
use diesel::deserialize::{self as de, FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::serialize::{self as ser, IsNull, Output, ToSql};
use diesel::sql_types::Text;
use diesel::sqlite::Sqlite;
use serde::Serialize;
use std::str::FromStr;

#[derive(FromSqlRow, AsExpression, Clone, Debug, Deref, Display, Serialize)]
#[diesel(sql_type = Text)]
pub struct Zoned(jiff::Zoned);

impl Zoned {
  pub fn now() -> Self {
    Self(jiff::Zoned::now())
  }
}

impl From<jiff::Zoned> for Zoned {
  fn from(zoned: jiff::Zoned) -> Zoned {
    Zoned(zoned)
  }
}

impl From<Zoned> for jiff::Zoned {
  fn from(zoned: Zoned) -> jiff::Zoned {
    zoned.0
  }
}

impl FromSql<Text, Sqlite> for Zoned {
  fn from_sql(bytes: <Sqlite as Backend>::RawValue<'_>) -> de::Result<Self> {
    let value = <String as FromSql<Text, Sqlite>>::from_sql(bytes)?;
    Ok(Zoned(jiff::Zoned::from_str(value.as_str())?))
  }
}

impl ToSql<Text, Sqlite> for Zoned
where
  String: ToSql<Text, Sqlite>,
{
  fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> ser::Result {
    out.set_value(self.to_string());
    Ok(IsNull::No)
  }
}
