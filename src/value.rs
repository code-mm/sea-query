//! Container for all SQL value types.
use std::fmt::Write;

#[cfg(feature="with-json")]
use std::str::from_utf8;
#[cfg(feature="with-json")]
use serde_json::Value as Json;

#[cfg(feature="with-chrono")]
use chrono::NaiveDateTime;

#[cfg(feature="with-uuid")]
use uuid::Uuid;

/// Value variants
#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Null,
    Bool(bool),
    TinyInt(i8),
    SmallInt(i16),
    Int(i32),
    BigInt(i64),
    TinyUnsigned(u8),
    SmallUnsigned(u16),
    Unsigned(u32),
    BigUnsigned(u64),
    Float(f32),
    Double(f64),
    // we want Value to be exactly 1 pointer sized, so anything larger should be boxed
    String(Box<String>),
    #[allow(clippy::box_vec)]
    Bytes(Box<Vec<u8>>),
    #[cfg(feature="with-json")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-json")))]
    Json(Box<Json>),
    #[cfg(feature="with-chrono")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
    DateTime(Box<NaiveDateTime>),
    #[cfg(feature="with-uuid")]
    #[cfg_attr(docsrs, doc(cfg(feature = "with-uuid")))]
    Uuid(Box<Uuid>),
}

pub trait ValueType {
    fn unwrap(v: Value) -> Self;
}

#[derive(Debug, PartialEq)]
pub struct Values(pub Vec<Value>);

impl Value {
    pub fn unwrap<T>(self) -> T
    where
        T: ValueType,
    {
        T::unwrap(self)
    }
}

macro_rules! type_to_value {
    ( $type: ty, $name: ident ) => {
        impl From<$type> for Value {
            fn from(x: $type) -> Value {
                Value::$name(x)
            }
        }

        impl From<Option<$type>> for Value {
            fn from(x: Option<$type>) -> Value {
                match x {
                    Some(v) => Value::$name(v),
                    None => Value::Null,
                }
            }
        }

        impl ValueType for $type {
            fn unwrap(v: Value) -> Self {
                match v {
                    Value::$name(x) => x,
                    _ => panic!("type error"),
                }
            }
        }

        impl ValueType for Option<$type> {
            fn unwrap(v: Value) -> Self {
                match v {
                    Value::$name(x) => Some(x),
                    _ => panic!("type error"),
                }
            }
        }
    };
}

macro_rules! type_to_box_value {
    ( $type: ty, $name: ident ) => {
        impl From<$type> for Value {
            fn from(x: $type) -> Value {
                Value::$name(Box::new(x))
            }
        }

        impl From<Option<$type>> for Value {
            fn from(x: Option<$type>) -> Value {
                match x {
                    Some(v) => Value::$name(Box::new(v)),
                    None => Value::Null,
                }
            }
        }

        impl ValueType for $type {
            fn unwrap(v: Value) -> Self {
                match v {
                    Value::$name(x) => *x,
                    _ => panic!("type error"),
                }
            }
        }

        impl ValueType for Option<$type> {
            fn unwrap(v: Value) -> Self {
                match v {
                    Value::$name(x) => Some(*x),
                    _ => panic!("type error"),
                }
            }
        }
    };
}

type_to_value!(bool, Bool);
type_to_value!(i8, TinyInt);
type_to_value!(i16, SmallInt);
type_to_value!(i32, Int);
type_to_value!(i64, BigInt);
type_to_value!(u8, TinyUnsigned);
type_to_value!(u16, SmallUnsigned);
type_to_value!(u32, Unsigned);
type_to_value!(u64, BigUnsigned);
type_to_value!(f32, Float);
type_to_value!(f64, Double);

impl<'a> From<&'a [u8]> for Value {
    fn from(x: &'a [u8]) -> Value {
        Value::Bytes(Box::<Vec<u8>>::new(x.into()))
    }
}

impl<'a> From<&'a str> for Value {
    fn from(x: &'a str) -> Value {
        let string: String = x.into();
        Value::String(Box::new(string))
    }
}

type_to_box_value!(Vec<u8>, Bytes);
type_to_box_value!(String, String);

#[cfg(feature="with-json")]
mod with_json {
    use super::*;

    #[cfg_attr(docsrs, doc(cfg(feature = "with-json")))]
    impl From<Json> for Value {
        fn from(x: Json) -> Value {
            Value::Json(Box::new(x))
        }
    }
}

#[cfg(feature="with-chrono")]
mod with_chrono {
    use super::*;

    #[cfg_attr(docsrs, doc(cfg(feature = "with-chrono")))]
    impl From<NaiveDateTime> for Value {
        fn from(x: NaiveDateTime) -> Value {
            Value::DateTime(Box::new(x))
        }
    }
}

#[cfg(feature="with-uuid")]
mod with_uuid {
    use super::*;

    #[cfg_attr(docsrs, doc(cfg(feature = "with-uuid")))]
    impl From<Uuid> for Value {
        fn from(x: Uuid) -> Value {
            Value::Uuid(Box::new(x))
        }
    }
}

impl Value {
    pub fn is_json(&self) -> bool {
        #[cfg(feature="with-json")]
        return matches!(self, Self::Json(_));
        #[cfg(not(feature="with-json"))]
        return false;
    }
    #[cfg(feature="with-json")]
    pub fn as_ref_json(&self) -> &Json {
        match self {
            Self::Json(v) => v.as_ref(),
            _ => panic!("not Value::Json"),
        }
    }
    #[cfg(not(feature="with-json"))]
    pub fn as_ref_json(&self) -> &bool {
        panic!("not Value::Json")
    }
}

impl Value {
    pub fn is_date_time(&self) -> bool {
        #[cfg(feature="with-chrono")]
        return matches!(self, Self::DateTime(_));
        #[cfg(not(feature="with-chrono"))]
        return false;
    }
    #[cfg(feature="with-chrono")]
    pub fn as_ref_date_time(&self) -> &NaiveDateTime {
        match self {
            Self::DateTime(v) => v.as_ref(),
            _ => panic!("not Value::DateTime"),
        }
    }
    #[cfg(not(feature="with-chrono"))]
    pub fn as_ref_date_time(&self) -> &bool {
        panic!("not Value::DateTime")
    }
}

impl Value {
    pub fn is_uuid(&self) -> bool {
        #[cfg(feature="with-uuid")]
        return matches!(self, Self::Uuid(_));
        #[cfg(not(feature="with-uuid"))]
        return false;
    }
    #[cfg(feature="with-uuid")]
    pub fn as_ref_uuid(&self) -> &Uuid {
        match self {
            Self::Uuid(v) => v.as_ref(),
            _ => panic!("not Value::Uuid"),
        }
    }
    #[cfg(not(feature="with-uuid"))]
    pub fn as_ref_uuid(&self) -> &bool {
        panic!("not Value::Uuid")
    }
}

/// Escape a SQL string literal
pub fn escape_string(string: &str) -> String {
    string
        .replace("\\", "\\\\")
        .replace("\"", "\\\"")
        .replace("'", "\\'")
        .replace("\0", "\\0")
        .replace("\x08", "\\b")
        .replace("\x09", "\\t")
        .replace("\x1a", "\\z")
        .replace("\n", "\\n")
        .replace("\r", "\\r")
}

/// Unescape a SQL string literal
pub fn unescape_string(input: &str) -> String {
    let mut escape = false;
    let mut output = String::new();
    for c in input.chars() {
        if !escape && c == '\\' {
            escape = true;
        } else if escape {
            write!(output, "{}", match c {
                '0' => '\0',
                'b' => '\x08',
                't' => '\x09',
                'z' => '\x1a',
                'n' => '\n',
                'r' => '\r',
                c => c,
            }).unwrap();
            escape = false;
        } else {
            write!(output, "{}", c).unwrap();
        }
    }
    output
}

/// Convert json value to value
#[cfg(feature="with-json")]
#[cfg_attr(docsrs, doc(cfg(feature = "with-json")))]
pub fn json_value_to_sea_value(v: &Json) -> Value {
    match v {
        Json::Null => Value::Null,
        Json::Bool(v) => Value::Int(v.to_owned().into()),
        Json::Number(v) =>
            if v.is_f64() {
                Value::Double(v.as_f64().unwrap())
            } else if v.is_i64() {
                Value::BigInt(v.as_i64().unwrap())
            } else if v.is_u64() {
                Value::BigUnsigned(v.as_u64().unwrap())
            } else {
                unimplemented!()
            },
        Json::String(v) => Value::String(Box::new(v.clone())),
        Json::Array(_) => unimplemented!(),
        Json::Object(v) => Value::Json(Box::new(Json::Object(v.clone()))),
    }
}

/// Convert value to json value
#[allow(clippy::many_single_char_names)]
#[cfg(feature="with-json")]
#[cfg_attr(docsrs, doc(cfg(feature = "with-json")))]
pub fn sea_value_to_json_value(v: &Value) -> Json {
    match v {
        Value::Null => Json::Null,
        Value::Bool(b) => Json::Bool(*b),
        Value::TinyInt(v) => (*v).into(),
        Value::SmallInt(v) => (*v).into(),
        Value::Int(v) => (*v).into(),
        Value::BigInt(v) => (*v).into(),
        Value::TinyUnsigned(v) => (*v).into(),
        Value::SmallUnsigned(v) => (*v).into(),
        Value::Unsigned(v) => (*v).into(),
        Value::BigUnsigned(v) => (*v).into(),
        Value::Float(v) => (*v).into(),
        Value::Double(v) => (*v).into(),
        Value::String(s) => Json::String(s.as_ref().clone()),
        Value::Bytes(s) => Json::String(from_utf8(s).unwrap().to_string()),
        Value::Json(v) => v.as_ref().clone(),
        #[cfg(feature="with-chrono")]
        Value::DateTime(v) => v.format("%Y-%m-%d %H:%M:%S").to_string().into(),
        #[cfg(feature="with-uuid")]
        Value::Uuid(v) => Json::String(v.to_string()),
    }
}

impl Values {
    pub fn iter(&self) -> impl Iterator<Item = &Value> {
        self.0.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escape_1() {
        let test = r#" "abc" "#;
        assert_eq!(escape_string(test), r#" \"abc\" "#.to_owned());
        assert_eq!(unescape_string(escape_string(test).as_str()), test);
    }

    #[test]
    fn test_escape_2() {
        let test = "a\nb\tc";
        assert_eq!(escape_string(test), "a\\nb\\tc".to_owned());
        assert_eq!(unescape_string(escape_string(test).as_str()), test);
    }

    #[test]
    fn test_escape_3() {
        let test = "a\\b";
        assert_eq!(escape_string(test), "a\\\\b".to_owned());
        assert_eq!(unescape_string(escape_string(test).as_str()), test);
    }

    #[test]
    fn test_escape_4() {
        let test = "a\"b";
        assert_eq!(escape_string(test), "a\\\"b".to_owned());
        assert_eq!(unescape_string(escape_string(test).as_str()), test);
    }

    #[test]
    fn test_value() {
        macro_rules! test_value {
            ( $type: ty, $val: literal ) => {
                let val: $type = $val;
                let v: Value = val.into();
                let out: $type = v.unwrap();
                assert_eq!(out, val);
            };
        }

        test_value!(u8, 255);
        test_value!(u16, 65535);
        test_value!(i8, 127);
        test_value!(i16, 32767);
        test_value!(i32, 1073741824);
        test_value!(i64, 8589934592);
    }

    #[test]
    fn test_box_value() {
        let val: String = "hello".to_owned();
        let v: Value = val.clone().into();
        let out: String = v.unwrap();
        assert_eq!(out, val);
    }
}