use std::{any::TypeId, collections::BTreeMap};

#[cfg(test)]
use crate::mock;

use crate::{
    proto::{self, Column, Identifier},
    python_type::PythonType,
};

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Default)]
pub struct Options {
    #[serde(default)]
    pub package: String,
    #[serde(default)]
    types: BTreeMap<String, TypeOverrideOptions>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Default)]
pub struct TypeOverrideOptions {
    pub name: String,
    pub import: Option<String>,
    pub encode: Option<String>,
    pub decode: Option<String>,
}

#[derive(Debug, serde::Serialize, Clone, PartialEq, Eq, Copy)]
pub enum Annotation {
    Exec,
    One,
    Many,
}

impl Options {
    pub fn from_request(req: &proto::GenerateRequest) -> Self {
        let json = String::from_utf8(
            req.settings
                .as_ref()
                .unwrap()
                .codegen
                .as_ref()
                .unwrap()
                .options
                .clone(),
        )
        .unwrap_or("{}".into());
        serde_json::from_str(&json).unwrap()
    }

    pub fn get_python_type(&self, column: &Column) -> PythonType {
        let mut r#type = match self.get_python_remapped_type(column) {
            Some(r#type) => r#type,
            _ => PythonType::from_pg_type(&column.r#type.as_ref().unwrap().name),
        };

        if column.is_array {
            for _ in 0..column.array_dims {
                r#type.annotation = format!("list[{}]", r#type.annotation);
            }
        }
        if !column.not_null {
            r#type.annotation = format!("{} | None", r#type.annotation);
        }

        r#type
    }

    pub fn get_python_remapped_type(&self, column: &Column) -> Option<PythonType> {
        let type_option = self
            .types
            .get(&column.r#type.as_ref().unwrap().name)?
            .clone();

        return Some(PythonType::from(type_option));
    }
}

#[test]
fn empty_option_is_deserializable() {
    let _: Options = serde_json::from_str("{}").unwrap();
}

#[test]
fn empty_option_get_python_type() {
    let options: Options = serde_json::from_str("{}").unwrap();
    let uuid_column = mock::uuid_column();
    let r#type = options.get_python_type(&uuid_column);

    assert_eq!(r#type.annotation, "uuid.UUID");
}

#[test]
fn overwritten_option_get_python_type() {
    let options: Options = serde_json::from_str(
        r#"{
            "types": {
                "geometry": {
                    "name": "shapely.Geometry",
                    "import": "import shapely",
                    "encode": "shapely.from_wkb"
                }
            }
        }"#,
    )
    .unwrap();
    let column = mock::geometry_column();
    let r#type = options.get_python_type(&column);

    assert_eq!(r#type.annotation, "shapely.Geometry");
}

#[test]
fn get_python_type_for_array_type() {
    let options: Options = serde_json::from_str(r#"{}"#).unwrap();
    let column = mock::int_array_column();
    let r#type = options.get_python_type(&column);

    assert_eq!(r#type.annotation, "list[int]");
}

#[test]
fn get_python_type_for_nested_array() {
    let options: Options = serde_json::from_str(r#"{}"#).unwrap();
    let column = mock::f64_matrix_column();
    let r#type = options.get_python_type(&column);
    assert_eq!(r#type.annotation, "list[list[float]]");
}

#[test]
fn get_python_type_for_nullable_string() {
    let options: Options = serde_json::from_str(r#"{}"#).unwrap();
    let column = mock::nullable_string_column();
    let r#type = options.get_python_type(&column);
    assert_eq!(r#type.annotation, "str | None");
}
