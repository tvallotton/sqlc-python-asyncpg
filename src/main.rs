use std::io::{self, BufRead, Cursor, Write};

use prost::Message;

use crate::{model_file::ModelFile, proto::GenerateRequest, response_builder::ResponseBuilder};

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
    let mut request = load_codgen_request();
    request.normalize_identifiers();

    let builder = ResponseBuilder::new(request);
    let response = builder.build();
    let mut buf = Vec::new();

    buf.reserve(response.encoded_len());

    response.encode(&mut buf).unwrap();

    match io::stdout().write_all(&buf) {
        Ok(result) => result,
        Err(_e) => std::process::exit(1),
    };
}
