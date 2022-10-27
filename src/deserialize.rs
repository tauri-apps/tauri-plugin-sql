use crate::plugin::Error;
use serde_json::Value as JsonValue;
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
    let v = match info.name() {
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
pub fn deserialize_col(
  row: &sqlx::postgres::PgRow,
  col: &sqlx::postgres::PgColumn,
  i: &usize,
) -> Result<JsonValue, Error> {
  let info = col.type_info();

  if info.is_null() {
    Ok(JsonValue::Null)
  } else {
    let v = match info.name() {
      "text" => JsonValue::String(row.try_get(i)?),
      "varchar" => JsonValue::String(row.try_get(i)?),
      "bool" => JsonValue::Bool(row.try_get(i)?),
      "date" => JsonValue::String(row.try_get(i)?),
      "time" => JsonValue::String(row.try_get(i)?),
      "timestamp" => JsonValue::String(row.try_get(i)?),
      "timestamptz" => JsonValue::String(row.try_get(i)?),
      "bytea" => JsonValue::String(base64::encode(row.try_get(i)?)),
      "int2" => JsonValue::Number(row.try_get(i)?.into()),
      "int4" => JsonValue::Number(row.try_get(i)?.into()),
      "int8" => JsonValue::Number(row.try_get(i)?.into()),
      "float4" => JsonValue::Number(row.try_get(i)?.into()),
      "float8" => JsonValue::Number(row.try_get(i)?.into()),
      "numeric" => JsonValue::Number(row.try_get(i)?.into()),
      _ => JsonValue::Null,
    };

    Ok(v)
  }
}

#[cfg(feature = "mysql")]
pub fn deserialize_col(
  row: &sqlx::mysql::MySqlRow,
  col: &sqlx::mysql::MySqlColumn,
  i: &usize,
) -> Result<JsonValue, Error> {
  let info = col.type_info();

  if info.is_null() {
    Ok(JsonValue::Null)
  } else {
    let v = match info.name() {
      "TIMESTAMP" => JsonValue::String(row.try_get(i).unwrap()),
      "DATE" => JsonValue::String(row.try_get(i).unwrap()),
      "TIME" => JsonValue::String(row.try_get(i).unwrap()),
      "DATETIME" => JsonValue::String(row.try_get(i).unwrap()),
      "NEWDATE" => JsonValue::String(row.try_get(i).unwrap()),
      "VARCHAR" => JsonValue::String(row.try_get(i).unwrap()),
      "VAR_STRING" => JsonValue::String(row.try_get(i).unwrap()),
      "STRING" => JsonValue::String(row.try_get(i).unwrap()),
      "TINY_BLOB" => JsonValue::String(base64::encode(row.try_get(i).unwrap())),
      "MEDIUM_BLOB" => JsonValue::String(base64::encode(row.try_get(i).unwrap())),
      "LONG_BLOB" => JsonValue::String(base64::encode(row.try_get(i).unwrap())),
      "BLOB" => JsonValue::String(base64::encode(row.try_get(i).unwrap())),
      "ENUM" => JsonValue::String(row.try_get(i).unwrap()),
      "SET" => JsonValue::String(row.try_get(i).unwrap()),
      "GEOMETRY" => JsonValue::String(base64::encode(row.try_get(i).unwrap())),
      "TINY" => JsonValue::Number(row.try_get(i).unwrap().into()),
      "SHORT" => JsonValue::Number(row.try_get(i).unwrap().into()),
      "LONG" => JsonValue::Number(row.try_get(i).unwrap().into()),
      "REAL" => JsonValue::Number(row.try_get(i).unwrap().into()),
      "LONGLONG" => JsonValue::Number(row.try_get(i).unwrap().into()),
      "INT24" => JsonValue::Number(row.try_get(i).unwrap().into()),
      "YEAR" => JsonValue::Number(row.try_get(i).unwrap().into()),
      "FLOAT" => JsonValue::Number(row.try_get(i).unwrap().into()),
      "DOUBLE" => JsonValue::Number(row.try_get(i).unwrap().into()),
      "BIT" => JsonValue::Number(row.try_get(i).unwrap().into()),
      _ => JsonValue::Null,
    };

    Ok(v)
  }
}

// let v = if info.is_null() {
//     JsonValue::Null
//   } else {
//     match info.name() {
//       "VARCHAR" | "STRING" | "TEXT" | "DATETIME" => {
//         if let Ok(s) = row.try_get(i) {
//           JsonValue::String(s)
//         } else {
//           JsonValue::Null
//         }
//       }
//       "BOOL" | "BOOLEAN" => {
//         if let Ok(b) = row.try_get(i) {
//           JsonValue::Bool(b)
//         } else {
//           let x: String = row.get(i);
//           JsonValue::Bool(x.to_lowercase() == "true")
//         }
//       }
//       "INT" | "NUMBER" | "INTEGER" | "BIGINT" | "INT8" => {
//         if let Ok(n) = row.try_get::<i64, usize>(i) {
//           JsonValue::Number(n.into())
//         } else {
//           JsonValue::Null
//         }
//       }
//       "REAL" => {
//         if let Ok(n) = row.try_get::<f64, usize>(i) {
//           JsonValue::from(n)
//         } else {
//           JsonValue::Null
//         }
//       }
//       // "JSON" => JsonValue::Object(row.get(i)),
//       "BLOB" => {
//         if let Ok(n) = row.try_get::<Vec<u8>, usize>(i) {
//           JsonValue::Array(n.into_iter().map(|n| JsonValue::Number(n.into())).collect())
//         } else {
//           JsonValue::Null
//         }
//       }
//       _ => JsonValue::Null,
//     }
