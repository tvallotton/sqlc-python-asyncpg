<<<<<<< HEAD
use std::io::{self, BufRead, Cursor, Write};
=======
use std::io::{self, BufRead, Cursor, Read, Write};
>>>>>>> main

use prost::Message;

use crate::{model_file::ModelFile, proto::GenerateRequest, response_builder::ResponseBuilder};

pub mod method;
pub mod method_builder;
#[cfg(test)]
pub mod mock;
pub mod model;
pub mod model_file;
pub mod model_file_generator;
<<<<<<< HEAD
=======
pub mod model_files;
>>>>>>> main
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
<<<<<<< HEAD
    let buffer = stdin.fill_buf().unwrap();

    match GenerateRequest::decode(&mut Cursor::new(buffer)) {
        Ok(request_deserialized_result) => request_deserialized_result,
        Err(_e) => std::process::exit(1),
=======
    let mut buffer: Vec<u8> = Vec::new();
    _ = stdin.read_to_end(&mut buffer).unwrap();

    match GenerateRequest::decode(&mut Cursor::new(buffer)) {
        Ok(request_deserialized_result) => request_deserialized_result,
        Err(e) => panic!("failed to load request: {}", e),
>>>>>>> main
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
<<<<<<< HEAD
        Err(_e) => std::process::exit(1),
=======
        Err(_e) => {
            panic!("failed to serialize response");
        }
>>>>>>> main
    };
}
