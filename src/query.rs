use std::sync::{LazyLock, OnceLock};

use regex::Regex;

use crate::{
    options::Options,
    proto::{Column, Identifier, Parameter},
    python_type::PythonType,
    utils::to_pascal_case,
};

#[derive(
    Clone, PartialEq, Eq, PartialOrd, Ord, ::prost::Message, serde::Serialize, serde::Deserialize,
)]
pub struct Query {
    #[prost(string, tag = "1")]
    pub text: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub cmd: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "4")]
    pub columns: ::prost::alloc::vec::Vec<Column>,
    #[prost(message, repeated, tag = "5")]
    pub params: ::prost::alloc::vec::Vec<Parameter>,
    #[prost(string, repeated, tag = "6")]
    pub comments: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag = "7")]
    pub filename: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "8")]
    pub insert_into_table: ::core::option::Option<Identifier>,
}

impl Query {
    pub fn model_name(&self) -> String {
        to_pascal_case(&self.name) + "Row"
    }

    pub fn qualified_model_name(&self) -> String {
        format!("models.{}.{}", self.module_name(), self.model_name())
    }

    pub fn model_type(&self, options: &Options) -> PythonType {
        PythonType {
            declaration: Some(self.model_name()),
            constructor: self.qualified_model_name(),
            annotation: self.qualified_model_name(),
            import: Some(format!("from {} import models", options.package)),
            encode: None,
            decode: None,
        }
    }

    pub fn module_name(&self) -> &str {
        let name = self
            .filename
            .split_once('.')
            .map(|(first, _)| first)
            .unwrap_or_else(|| &self.filename);

        return name;
    }

    pub fn namespace_name(&self) -> String {
        static PATTERN: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(r"\s*namespace:\s*([\w.]+)\s*").unwrap());

        for val in &self.comments {
            if let Some(capture) = PATTERN.captures(&val) {
                return capture.get(1).unwrap().as_str().into();
            }
        }

        self.filename
            .strip_suffix(".sql")
            .unwrap_or(&self.filename)
            .into()
    }
}

#[test]
fn model_name() {
    let model = crate::mock::query_get_all_posts();
    assert_eq!(model.model_name(), "GetAllPostsRow");
}

#[test]
fn qualified_model_name() {
    let model = crate::mock::query_get_all_posts();
    assert_eq!(model.qualified_model_name(), "foo.GetAllPostsRow");
}
