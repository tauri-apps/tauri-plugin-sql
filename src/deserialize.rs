use crate::plugin::Error;
use serde_json::Value as JsonValue;
#[allow(unused_imports)]
use sqlx::{Column, Row, TypeInfo};

#[cfg(feature = "sqlite")]
pub fn deserialize_col<'a>(
  row: &'a sqlx::sqlite::SqliteRow,
  col: &'a sqlx::sqlite::SqliteColumn,
  i: &'a usize,
) -> Result<JsonValue, Error> {
  let info = col.type_info();

  if info.is_null() {
    Ok(JsonValue::Null)
  } else {
    let v = match info.name().to_uppercase().as_str() {
      "TEXT" => JsonValue::String(row.try_get(i)?),
      "BLOB" => {
        let v: String = row.try_get(i)?;
        JsonValue::String(base64::encode(v))
      }
      "INTEGER" => {
        if let Ok(v) = row.try_get::<i64, &usize>(i) {
          return Ok(JsonValue::Number(v.into()));
        }
        if let Ok(v) = row.try_get::<i32, &usize>(i) {
          return Ok(JsonValue::Number(v.into()));
        }
        if let Ok(v) = row.try_get::<i16, &usize>(i) {
          return Ok(JsonValue::Number(v.into()));
        }
        if let Ok(v) = row.try_get::<i8, &usize>(i) {
          return Ok(JsonValue::Number(v.into()));
        }

        return Err(Error::NumericDecoding(
          info.name().to_string(),
          String::from("Sqlite"),
        ));
      }
      "REAL" => {
        let v: i64 = row.try_get(i)?;
        JsonValue::Number(v.into())
      }
      _ => JsonValue::Null,
    };

    Ok(v)
  }
}

#[cfg(feature = "postgres")]
pub fn deserialize_col<'a>(
  row: &'a sqlx::postgres::PgRow,
  col: &'a sqlx::postgres::PgColumn,
  i: &'a usize,
) -> Result<JsonValue, Error> {
  let info = col.type_info();

  if info.is_null() {
    Ok(JsonValue::Null)
  } else {
    Ok(match info.name().to_uppercase().as_str() {
      "TEXT" => JsonValue::String(row.try_get(i)?),
      "VARCHAR" => JsonValue::String(row.try_get(i)?),
      "BOOL" => JsonValue::Bool(row.try_get(i)?),
      "DATE" => JsonValue::String(row.try_get(i)?),
      "TIME" => JsonValue::String(row.try_get(i)?),
      "TIMESTAMP" => JsonValue::String(row.try_get(i)?),
      "TIMESTAMPTZ" => JsonValue::String(row.try_get(i)?),
      "BYTEA" => JsonValue::String(base64::encode(row.try_get::<String, &usize>(i)?)),
      "INT2" => JsonValue::Number(row.try_get::<i16, &usize>(i)?.into()),
      "INT4" => JsonValue::Number(row.try_get::<i32, &usize>(i)?.into()),
      "INT8" => JsonValue::Number(row.try_get::<i64, &usize>(i)?.into()),
      "FLOAT4" => JsonValue::Number(row.try_get::<i32, &usize>(i)?.into()),
      "FLOAT8" => JsonValue::Number(row.try_get::<i64, &usize>(i)?.into()),
      "NUMERIC" => {
        if let Ok(v) = row.try_get::<i64, &usize>(i) {
          return Ok(JsonValue::Number(v.into()));
        }
        if let Ok(v) = row.try_get::<i32, &usize>(i) {
          return Ok(JsonValue::Number(v.into()));
        }
        if let Ok(v) = row.try_get::<i16, &usize>(i) {
          return Ok(JsonValue::Number(v.into()));
        }
        if let Ok(v) = row.try_get::<i8, &usize>(i) {
          return Ok(JsonValue::Number(v.into()));
        }

        return Err(Error::NumericDecoding(
          info.name().to_string(),
          String::from("Postgres"),
        ));
      }
      _ => JsonValue::Null,
    })
  }
}

#[cfg(feature = "mysql")]
pub fn deserialize_col<'a>(
  row: &'a sqlx::mysql::MySqlRow,
  col: &'a sqlx::mysql::MySqlColumn,
  i: &'a usize,
) -> Result<JsonValue, Error> {
  let info = col.type_info();

  if info.is_null() {
    Ok(JsonValue::Null)
  } else {
    let v = match info.name().to_uppercase().as_str() {
      "TIMESTAMP" => JsonValue::String(row.try_get(i)?),
      "DATE" => JsonValue::String(row.try_get(i)?),
      "TIME" => JsonValue::String(row.try_get(i)?),
      "DATETIME" => JsonValue::String(row.try_get(i)?),
      "NEWDATE" => JsonValue::String(row.try_get(i)?),
      "VARCHAR" => JsonValue::String(row.try_get(i)?),
      "VAR_STRING" => JsonValue::String(row.try_get(i)?),
      "STRING" => JsonValue::String(row.try_get(i)?),
      "TINY_BLOB" => JsonValue::String(base64::encode(row.try_get::<String, &usize>(i)?)),
      "MEDIUM_BLOB" => JsonValue::String(base64::encode(row.try_get::<String, &usize>(i)?)),
      "LONG_BLOB" => JsonValue::String(base64::encode(row.try_get::<String, &usize>(i)?)),
      "BLOB" => JsonValue::String(base64::encode(row.try_get::<String, &usize>(i)?)),
      "ENUM" => JsonValue::String(row.try_get(i)?),
      "SET" => JsonValue::String(row.try_get(i)?),
      "GEOMETRY" => JsonValue::String(base64::encode(row.try_get::<String, &usize>(i)?)),
      "TINY" | "TINYINT" => JsonValue::Number(row.try_get::<i8, &usize>(i)?.into()),
      "SMALL" | "SMALLINT" => JsonValue::Number(row.try_get::<i16, &usize>(i)?.into()),
      // really only takes 24-bits
      "MEDIUM" | "MEDIUMINT" => JsonValue::Number(row.try_get::<i32, &usize>(i)?.into()),
      // 32-bit primitive
      "INT" => JsonValue::Number(row.try_get::<i32, &usize>(i)?.into()),
      "BIGINT" => JsonValue::Number(row.try_get::<i64, &usize>(i)?.into()),
      "REAL" => JsonValue::Number(row.try_get::<i64, &usize>(i)?.into()),
      "YEAR" => JsonValue::Number(row.try_get::<i16, &usize>(i)?.into()),
      "FLOAT" => JsonValue::Number(row.try_get::<i32, &usize>(i)?.into()),
      "DOUBLE" => JsonValue::Number(row.try_get::<i64, &usize>(i)?.into()),
      "BIT" => JsonValue::Number(row.try_get::<i8, &usize>(i)?.into()),
      _ => JsonValue::Null,
    };

    Ok(v)
  }
}
