#[cfg(test)]
use crate::mock;
use crate::model_file::ModelFile;
use std::collections::BTreeMap;

pub struct ModelFileGenerator {
    pub default_schema: String,
    pub model_files: BTreeMap<String, ModelFile>,
}

impl ModelFileGenerator {
    pub fn new(default_schema: &str) -> Self {
        ModelFileGenerator {
            default_schema: default_schema.into(),
            model_files: Default::default(),
        }
    }
}

#[test]
fn model_file_generator_generate() {
    let mut generator = ModelFileGenerator::new("public");
    let mut public = ModelFile::default();
    public.models.push(mock::user_model());
    generator.model_files.insert("public".into(), public);
}
