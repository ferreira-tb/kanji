use derive_more::{Deref, Display};
use diesel::backend::Backend;
use diesel::deserialize::{self as de, FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::serialize::{self as ser, IsNull, Output, ToSql};
use diesel::sql_types::Text;
use diesel::sqlite::Sqlite;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

macro_rules! impl_uuid {
  { $name:ident, $new:ident } => {
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
      PartialOrd,
      Ord,
      Hash,
      Deserialize,
      Serialize,
    )]
    #[diesel(sql_type = Text)]
    pub struct $name(Uuid);

    impl $name {
      pub fn new() -> Self {
        Self(Uuid::$new())
      }
    }

    impl Default for $name {
      fn default() -> Self {
        Self::new()
      }
    }

    impl FromSql<Text, Sqlite> for $name {
      fn from_sql(bytes: <Sqlite as Backend>::RawValue<'_>) -> de::Result<Self> {
        let value = <String as FromSql<Text, Sqlite>>::from_sql(bytes)?;
        let uuid = Uuid::try_parse(value.as_str())?;
        Ok($name(uuid))
      }
    }

    impl ToSql<Text, Sqlite> for $name
    where
      String: ToSql<Text, Sqlite>,
    {
      fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> ser::Result {
        out.set_value(self.to_string());
        Ok(IsNull::No)
      }
    }
  };
}

macro_rules! impl_uuid_v7 {
  ($($name:ident),+ $(,)?) => {
    $(impl_uuid! { $name, now_v7 })+
  }
}

impl_uuid_v7!(QuizAnswerId);
