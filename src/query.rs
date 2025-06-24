use crate::{
    proto::{Column, Identifier, Parameter},
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
}

#[test]
fn model_name() {
    let model = crate::mock::query_get_all_posts();

    assert_eq!(model.model_name(), "GetAllPostsRow");
}
