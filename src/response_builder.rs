use std::collections::BTreeMap;

use crate::{model_file::ModelFile, proto::Catalog, query_class::QueryNamespace};

#[derive(Default)]
pub struct ResponseBuilder {
    root: QueryNamespace,
    model_files: BTreeMap<String, ModelFile>,
}

impl ResponseBuilder {
    pub fn add(&mut self) {}
}
