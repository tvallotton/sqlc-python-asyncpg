#[cfg(test)]
use crate::mock;
use crate::{
    options::Options,
    proto::{Column, Identifier},
    python_type::PythonType,
    utils::{to_pascal_case, to_snake_case},
};

#[derive(
    Clone, PartialEq, Eq, PartialOrd, Ord, ::prost::Message, serde::Serialize, serde::Deserialize,
)]
pub struct Table {
    #[prost(message, optional, tag="1")]
    pub rel: ::core::option::Option<Identifier>,
    #[prost(message, repeated, tag="2")]
    pub columns: ::prost::alloc::vec::Vec<Column>,
    #[prost(string, tag="3")]
    pub comment: ::prost::alloc::string::String,
}

impl Table {
    // TODO: allow comments to rename
    pub fn model_name(&self) -> String {
        to_pascal_case(&self.rel.as_ref().unwrap().name)
    }

    pub fn qualified_model_name(&self) -> String {
        format!("models.{}.{}", self.schema_name(), self.model_name())
    }

    pub fn model_type(&self, options: &Options) -> PythonType {
        PythonType {
            constructor: self.qualified_model_name(),
            declaration: Some(self.model_name()),
            annotation: self.qualified_model_name(),
            import: Some(format!("from {} import models", options.package)),
            encode: None,
            decode: None,
        }
    }

    fn schema_name(&self) -> &str {
        &self.rel.as_ref().unwrap().schema
    }
}

#[test]
fn table_model_name() {
    let user_table = mock::user_table();
    assert_eq!(user_table.model_name(), "User");
}
