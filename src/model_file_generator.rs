#[cfg(test)]
use crate::mock;
use crate::{model_file::ModelFile, utils::to_snake_case};
use std::collections::BTreeMap;

pub struct ModelFileGenerator {
    pub package: String,
    pub default_schema: String,
    pub model_files: BTreeMap<String, ModelFile>,
}

impl ModelFileGenerator {
    pub fn new(package: &str, default_schema: &str) -> Self {
        ModelFileGenerator {
            package: package.into(),
            default_schema: to_snake_case(default_schema),
            model_files: Default::default(),
        }
    }

    pub fn init_file_contents(&self) -> String {
        let model_file = self.model_files.get(&self.default_schema);
        minijinja::render!(include_str!("../templates/model_init.py.jinja2"), package => self.package, default_schema => self.default_schema, models => model_file.unwrap().models )
    }
}

#[test]
fn model_file_generator_generate() {
    let mut generator = ModelFileGenerator::new("app", "public");
    let mut public = ModelFile::default();
    public.models.push(mock::user_model());
    generator.model_files.insert("public".into(), public);

    let expected = r#"from app.models.public import (
    User,
)"#;
    assert_eq!(generator.init_file_contents(), expected);
}
