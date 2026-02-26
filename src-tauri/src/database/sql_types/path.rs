use anyhow::anyhow;
use diesel::backend::Backend;
use diesel::deserialize::{self as de, FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::serialize::{self as ser, IsNull, Output, ToSql};
use diesel::sql_types::Text;
use diesel::sqlite::Sqlite;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::ops::Deref;
use std::path::PathBuf;

#[derive(FromSqlRow, AsExpression, Clone, Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[diesel(sql_type = Text)]
pub struct SqlPath(PathBuf);

impl Deref for SqlPath {
  type Target = std::path::Path;

  fn deref(&self) -> &Self::Target {
    self.0.as_path()
  }
}

impl<T> From<T> for SqlPath
where
  T: AsRef<std::path::Path>,
{
  fn from(path: T) -> Self {
    Self(path.as_ref().to_path_buf())
  }
}

impl fmt::Display for SqlPath {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0.to_string_lossy())
  }
}

impl FromSql<Text, Sqlite> for SqlPath {
  fn from_sql(bytes: <Sqlite as Backend>::RawValue<'_>) -> de::Result<Self> {
    let value = <String as FromSql<Text, Sqlite>>::from_sql(bytes)?;
    Ok(SqlPath(PathBuf::from(value)))
  }
}

impl ToSql<Text, Sqlite> for SqlPath
where
  String: ToSql<Text, Sqlite>,
{
  fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> ser::Result {
    let Some(path) = self.to_str() else {
      return Err(Into::into(anyhow!("invalid path: {self}")));
    };

    out.set_value(path);
    Ok(IsNull::No)
  }
}
