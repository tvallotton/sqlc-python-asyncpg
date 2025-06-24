#[cfg(test)]
use crate::mock;

use crate::{options::TypeOverrideOptions, proto::Column};

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize, PartialOrd, Ord)]
pub struct PythonType {
    pub declaration: Option<String>,

    pub constructor: String,

    #[serde(default)]
    pub annotation: String,

    #[serde(default)]
    pub import: Option<String>,

    #[serde(default)]
    pub encode: Option<String>,

    #[serde(default)]
    pub decode: Option<String>,
}

macro_rules! new_type {
    ($name:literal) => {
        new_type!($name, None)
    };
    ($name:literal,$import:tt) => {{
        let import: Option<&'static str> = Option::from($import);
        crate::python_type::PythonType {
            declaration: None,
            constructor: $name.into(),
            annotation: $name.into(),
            import: import.map(String::from),
            encode: None,
            decode: None,
        }
    }};
}

impl PythonType {
    pub fn imports(&self) -> impl Iterator<Item = &str> {
        self.import.as_deref().into_iter()
    }

    pub fn from_pg_type(name: &str) -> Self {
        match name {
            "anyarray" | "anyenum" => new_type!("list[typing.Any]", "import typing"),
            "anyrange" => new_type!("asyncpg.Range", "import asyncpg"),
            "anymultirange" => new_type!("list[asyncpg.Range]", "import asyncpg"),
            "record" => new_type!("asyncpg.Record", "import asyncpg"),
            "bit" | "varbit" => new_type!("asyncpg.BitString", "import asyncpg"),
            "bool" | "boolean" => new_type!("bool"),
            "box" => new_type!("asyncpg.Box", "import asyncpg"),
            "bytea" => new_type!("bytes"),
            "macaddr" | "char" | "name" | "varchar" | "text" | "xml" => new_type!("str"),
            "cidr" => new_type!(
                "ipaddress.IPv4Network | ipaddress.IPv6Network",
                "import ipaddress"
            ),
            "circle" => new_type!("asyncpg.types.Circle", "import asyncpg.types"),
            "date" => new_type!("datetime.date", "import datetime"),

            "time" | "time with time zone" | "timetz" => {
                new_type!("datetime.time", "import datetime")
            }
            "timestamp" | "timestamp with time zone" | "timestamptz" => {
                new_type!("datetime.datetime", "import datetime")
            }
            "interval" => new_type!("datetime.timedelta", "import datetime"),
            "float" | "double" | "precision" => new_type!("float"),
            "smallint" | "integer" | "bigint" => new_type!("int"),
            "numeric" => new_type!("decimal.Decimal", "import decimal"),
            "json" | "jsonb" | "money" => new_type!("str"),
            "line" => new_type!("asyncpg.Line", "import asyncpg"),
            "lseg" => new_type!("asyncpg.LineSegment", "import asyncpg"),
            "path" => new_type!("asyncpg.types.Path", "import asyncpg.types"),
            "point" => new_type!("asyncpg.types.Point", "import asyncpg.types"),
            "polygon" => new_type!("asyncpg.types.Polygon", "import asyncpg.types"),
            "uuid" => new_type!("uuid.UUID", "import uuid"),
            "tid" => new_type!("tuple"),
            _ => new_type!("str"),
        }
    }
}

impl From<TypeOverrideOptions> for PythonType {
    fn from(value: TypeOverrideOptions) -> Self {
        PythonType {
            constructor: value.name.clone(),
            declaration: None,
            annotation: value.name,
            encode: value.encode.clone(),
            decode: value.decode.clone(),
            import: value.import,
        }
    }
}

#[test]
fn parse_type_from_json() {
    let json = r#"{ "name": "dict" }"#;
    let type_option = serde_json::from_str::<TypeOverrideOptions>(json).unwrap();
    let _ = PythonType::from(type_option);
}

#[test]
fn test_type_imports() {
    let json = r#"{ "name": "uuid.UUID", "import": "import uuid" }"#;
    let type_option = serde_json::from_str::<TypeOverrideOptions>(json).unwrap();
    let type_ = PythonType::from(type_option);
    assert!(type_.imports().any(|import| import == "import uuid"));
}

#[test]
fn test_type_new() {
    let r#type = new_type!("uuid.UUID", "import, uuid");

    assert_eq!(r#type.constructor, "uuid.UUID")
}
