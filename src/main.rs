use std::io::{self, BufRead, Cursor};

use prost::Message;

use crate::proto::GenerateRequest;

pub mod method;
pub mod method_builder;
#[cfg(test)]
pub mod mock;
pub mod model;
pub mod model_file;
pub mod model_file_generator;
pub mod normalization;
pub mod options;
pub mod proto;
pub mod python_type;
pub mod query;
pub mod query_class;
pub mod response_builder;
pub mod table;
pub mod utils;

pub fn load_codgen_request() -> GenerateRequest {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let buffer = stdin.fill_buf().unwrap();

    match GenerateRequest::decode(&mut Cursor::new(buffer)) {
        Ok(request_deserialized_result) => request_deserialized_result,
        Err(_e) => std::process::exit(1),
    }
}

fn main() {
    let codegen = load_codgen_request();
}
