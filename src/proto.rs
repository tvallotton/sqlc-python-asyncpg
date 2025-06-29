use crate::{query::Query, table::Table};

#[derive(
    Clone, PartialEq, Eq, PartialOrd, Ord, ::prost::Message, serde::Serialize, serde::Deserialize,
)]
pub struct File {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="2")]
    pub contents: ::prost::alloc::vec::Vec<u8>,
}
#[derive(
    Clone, PartialEq, Eq, PartialOrd, Ord, ::prost::Message, serde::Serialize, serde::Deserialize,
)]
pub struct Settings {
    #[prost(string, tag="1")]
    pub version: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub engine: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="3")]
    pub schema: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="4")]
    pub queries: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, optional, tag="12")]
    pub codegen: ::core::option::Option<Codegen>,
}
#[derive(
    Clone, PartialEq, Eq, PartialOrd, Ord, ::prost::Message, serde::Serialize, serde::Deserialize,
)]
pub struct Codegen {
    #[prost(string, tag="1")]
    pub out: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub plugin: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="3")]
    pub options: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, repeated, tag="4")]
    pub env: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, optional, tag="5")]
    pub process: ::core::option::Option<codegen::Process>,
    #[prost(message, optional, tag="6")]
    pub wasm: ::core::option::Option<codegen::Wasm>,
}
/// Nested message and enum types in `Codegen`.
pub mod codegen {
    #[derive(
    Clone, PartialEq, Eq, PartialOrd, Ord, ::prost::Message, serde::Serialize, serde::Deserialize,
)]
    pub struct Process {
        #[prost(string, tag="1")]
        pub cmd: ::prost::alloc::string::String,
    }
    #[derive(
    Clone, PartialEq, Eq, PartialOrd, Ord, ::prost::Message, serde::Serialize, serde::Deserialize,
)]
    pub struct Wasm {
        #[prost(string, tag="1")]
        pub url: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub sha256: ::prost::alloc::string::String,
    }
}
#[derive(
    Clone, PartialEq, Eq, PartialOrd, Ord, ::prost::Message, serde::Serialize, serde::Deserialize,
)]
pub struct Catalog {
    #[prost(string, tag="1")]
    pub comment: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub default_schema: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="4")]
    pub schemas: ::prost::alloc::vec::Vec<Schema>,
}
#[derive(
    Clone, PartialEq, Eq, PartialOrd, Ord, ::prost::Message, serde::Serialize, serde::Deserialize,
)]
pub struct Schema {
    #[prost(string, tag="1")]
    pub comment: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="3")]
    pub tables: ::prost::alloc::vec::Vec<Table>,
    #[prost(message, repeated, tag="4")]
    pub enums: ::prost::alloc::vec::Vec<Enum>,
    #[prost(message, repeated, tag="5")]
    pub composite_types: ::prost::alloc::vec::Vec<CompositeType>,
}
#[derive(
    Clone, PartialEq, Eq, PartialOrd, Ord, ::prost::Message, serde::Serialize, serde::Deserialize,
)]
pub struct CompositeType {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub comment: ::prost::alloc::string::String,
}
#[derive(
    Clone, PartialEq, Eq, PartialOrd, Ord, ::prost::Message, serde::Serialize, serde::Deserialize,
)]
pub struct Enum {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="2")]
    pub vals: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag="3")]
    pub comment: ::prost::alloc::string::String,
}

#[derive(
    Clone, PartialEq, Eq, PartialOrd, Ord, ::prost::Message, serde::Serialize, serde::Deserialize,
)]
pub struct Identifier {
    #[prost(string, tag="1")]
    pub catalog: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub schema: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub name: ::prost::alloc::string::String,
}
#[derive(
    Clone, PartialEq, Eq, PartialOrd, Ord, ::prost::Message, serde::Serialize, serde::Deserialize,
)]
pub struct Column {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(bool, tag="3")]
    pub not_null: bool,
    #[prost(bool, tag="4")]
    pub is_array: bool,
    #[prost(string, tag="5")]
    pub comment: ::prost::alloc::string::String,
    #[prost(int32, tag="6")]
    pub length: i32,
    #[prost(bool, tag="7")]
    pub is_named_param: bool,
    #[prost(bool, tag="8")]
    pub is_func_call: bool,
    /// XXX: Figure out what PostgreSQL calls `foo.id`
    #[prost(string, tag="9")]
    pub scope: ::prost::alloc::string::String,
    #[prost(message, optional, tag="10")]
    pub table: ::core::option::Option<Identifier>,
    #[prost(string, tag="11")]
    pub table_alias: ::prost::alloc::string::String,
    #[prost(message, optional, tag="12")]
    pub r#type: ::core::option::Option<Identifier>,
    #[prost(bool, tag="13")]
    pub is_sqlc_slice: bool,
    #[prost(message, optional, tag="14")]
    pub embed_table: ::core::option::Option<Identifier>,
    #[prost(string, tag="15")]
    pub original_name: ::prost::alloc::string::String,
    #[prost(bool, tag="16")]
    pub unsigned: bool,
    #[prost(int32, tag="17")]
    pub array_dims: i32,
}

#[derive(
    Clone, PartialEq, Eq, PartialOrd, Ord, ::prost::Message, serde::Serialize, serde::Deserialize,
)]
pub struct Parameter {
    #[prost(int32, tag="1")]
    pub number: i32,
    #[prost(message, optional, tag="2")]
    pub column: ::core::option::Option<Column>,
}
#[derive(
    Clone, PartialEq, Eq, PartialOrd, Ord, ::prost::Message, serde::Serialize, serde::Deserialize,
)]
pub struct GenerateRequest {
    #[prost(message, optional, tag="1")]
    pub settings: ::core::option::Option<Settings>,
    #[prost(message, optional, tag="2")]
    pub catalog: ::core::option::Option<Catalog>,
    #[prost(message, repeated, tag="3")]
    pub queries: ::prost::alloc::vec::Vec<Query>,
    #[prost(string, tag="4")]
    pub sqlc_version: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="5")]
    pub plugin_options: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="6")]
    pub global_options: ::prost::alloc::vec::Vec<u8>,
}
#[derive(
    Clone, PartialEq, Eq, PartialOrd, Ord, ::prost::Message, serde::Serialize, serde::Deserialize,
)]
pub struct GenerateResponse {
    #[prost(message, repeated, tag="1")]
    pub files: ::prost::alloc::vec::Vec<File>,
}
