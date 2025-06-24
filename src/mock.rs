#[cfg(test)]
use crate::proto::Column;
use crate::{model::Model, options::Options, query::Query, table::Table};

#[cfg(test)]
pub fn uuid_column() -> Column {
    serde_json::from_str(
        r#" {
      "name": "id",
      "not_null": true,
      "is_array": false,
      "comment": "",
      "length": -1,
      "is_named_param": false,
      "is_func_call": false,
      "scope": "",
      "table": { "catalog": "", "schema": "", "name": "post" },
      "table_alias": "",
      "type": { "catalog": "", "schema": "", "name": "uuid" },
      "is_sqlc_slice": false,
      "embed_table": null,
      "original_name": "",
      "unsigned": false,
      "array_dims": 0
    }"#,
    )
    .unwrap()
}

#[cfg(test)]
pub fn geometry_column() -> Column {
    serde_json::from_str(
        r#" {
      "name": "location",
      "not_null": true,
      "is_array": false,
      "comment": "",
      "length": -1,
      "is_named_param": false,
      "is_func_call": false,
      "scope": "",
      "table": { "catalog": "", "schema": "", "name": "geolocation" },
      "table_alias": "",
      "type": { "catalog": "", "schema": "", "name": "geometry" },
      "is_sqlc_slice": false,
      "embed_table": null,
      "original_name": "",
      "unsigned": false,
      "array_dims": 0
    }"#,
    )
    .unwrap()
}

#[cfg(test)]
pub fn int_array_column() -> Column {
    serde_json::from_str(
        r#" {
      "name": "favorite_numbers",
      "not_null": true,
      "is_array": true,
      "comment": "",
      "length": -1,
      "is_named_param": false,
      "is_func_call": false,
      "scope": "",
      "table": { "catalog": "", "schema": "", "name": "profile" },
      "table_alias": "",
      "type": { "catalog": "", "schema": "", "name": "integer" },
      "is_sqlc_slice": false,
      "embed_table": null,
      "original_name": "",
      "unsigned": false,
      "array_dims": 1
    }"#,
    )
    .unwrap()
}

#[cfg(test)]
pub fn f64_matrix_column() -> Column {
    serde_json::from_str(
        r#" {
      "name": "covariance_matrix",
      "not_null": true,
      "is_array": true,
      "comment": "",
      "length": -1,
      "is_named_param": false,
      "is_func_call": false,
      "scope": "",
      "table": { "catalog": "", "schema": "", "name": "profile" },
      "table_alias": "",
      "type": { "catalog": "", "schema": "", "name": "double" },
      "is_sqlc_slice": false,
      "embed_table": null,
      "original_name": "",
      "unsigned": false,
      "array_dims": 2
    }"#,
    )
    .unwrap()
}

#[cfg(test)]
pub fn nullable_string_column() -> Column {
    serde_json::from_str(
        r#" {
      "name": "phone",
      "not_null": false,
      "is_array": false,
      "comment": "",
      "length": -1,
      "is_named_param": false,
      "is_func_call": false,
      "scope": "",
      "table": { "catalog": "", "schema": "", "name": "profile" },
      "table_alias": "",
      "type": { "catalog": "", "schema": "", "name": "text" },
      "is_sqlc_slice": false,
      "embed_table": null,
      "original_name": "",
      "unsigned": false,
      "array_dims": 2
    }"#,
    )
    .unwrap()
}

pub fn user_table() -> Table {
    let mut table: Table = serde_json::from_str(
        r#" {
      "rel": { "catalog": "", "schema": "", "name": "user" },
      "columns": [
        {
          "name": "id",
          "not_null": true,
          "is_array": false,
          "comment": "",
          "length": -1,
          "is_named_param": false,
          "is_func_call": false,
          "scope": "",
          "table": { "catalog": "", "schema": "", "name": "user" },
          "table_alias": "",
          "type": { "catalog": "", "schema": "", "name": "uuid" },
          "is_sqlc_slice": false,
          "embed_table": null,
          "original_name": "",
          "unsigned": false,
          "array_dims": 0
        },
        {
          "name": "email",
          "not_null": true,
          "is_array": false,
          "comment": "",
          "length": -1,
          "is_named_param": false,
          "is_func_call": false,
          "scope": "",
          "table": { "catalog": "", "schema": "", "name": "user" },
          "table_alias": "",
          "type": { "catalog": "", "schema": "", "name": "text" },
          "is_sqlc_slice": false,
          "embed_table": null,
          "original_name": "",
          "unsigned": false,
          "array_dims": 0
        }
      ],
      "comment": ""
    }"#,
    )
    .unwrap();
    table.normalize_identifiers("public");
    table
}

pub fn user_review_table() -> Table {
    let mut table: Table = serde_json::from_str(
        r#" {
      "rel": { "catalog": "", "schema": "post", "name": "user_review" },
      "columns": [
        {
          "name": "id",
          "not_null": true,
          "is_array": false,
          "comment": "",
          "length": -1,
          "is_named_param": false,
          "is_func_call": false,
          "scope": "",
          "table": { "catalog": "", "schema": "post", "name": "user_review" },
          "table_alias": "",
          "type": { "catalog": "", "schema": "", "name": "uuid" },
          "is_sqlc_slice": false,
          "embed_table": null,
          "original_name": "",
          "unsigned": false,
          "array_dims": 0
        },
        {
          "name": "url",
          "not_null": true,
          "is_array": false,
          "comment": "",
          "length": -1,
          "is_named_param": false,
          "is_func_call": false,
          "scope": "",
          "table": { "catalog": "", "schema": "post", "name": "user_review" },
          "table_alias": "",
          "type": { "catalog": "", "schema": "", "name": "text" },
          "is_sqlc_slice": false,
          "embed_table": null,
          "original_name": "",
          "unsigned": false,
          "array_dims": 0
        }
      ],
      "comment": ""
    }"#,
    )
    .unwrap();
    table.normalize_identifiers("public");
    table
}

pub fn query_get_all_posts() -> Query {
    let mut query: Query = serde_json::from_str(
        r#"{
      "text": "select\n    post.id, post.author_id, post.title\nfrom\n    \"post\"",
      "name": "get_all_posts",
      "cmd": ":many",
      "columns": [
        {
          "name": "post",
          "not_null": false,
          "is_array": false,
          "comment": "",
          "length": -1,
          "is_named_param": false,
          "is_func_call": false,
          "scope": "",
          "table": null,
          "table_alias": "",
          "type": { "catalog": "", "schema": "", "name": "" },
          "is_sqlc_slice": false,
          "embed_table": { "catalog": "", "schema": "", "name": "post" },
          "original_name": "",
          "unsigned": false,
          "array_dims": 0
        }
      ],
      "params": [],
      "comments": [],
      "filename": "foo.sql",
      "insert_into_table": null
    }"#,
    )
    .unwrap();
    query.normalize_identifiers("public");
    query
}

pub fn user_model() -> Model {
    Model::from_table(&user_table(), &Options::default())
}
