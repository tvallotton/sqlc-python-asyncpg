#[cfg(test)]
use crate::mock;
use crate::{model_file::ModelFile, model_files::ModelFiles, proto::File, utils::to_snake_case};
use std::collections::BTreeMap;

pub struct ModelFileGenerator {
    pub package: String,
    pub default_schema: String,
    pub model_files: ModelFiles,
}

impl ModelFileGenerator {
    pub fn new(package: &str, default_schema: &str) -> Self {
        ModelFileGenerator {
            package: package.into(),
            default_schema: to_snake_case(default_schema),
            model_files: Default::default(),
        }
    }

    pub fn init_file_contents(&self) -> Vec<u8> {
        let model_file = self.model_files.model_files.get(&self.default_schema);
        minijinja::render!(include_str!("../templates/model_init.py.jinja2"),
            package => self.package,
            default_schema => self.default_schema,
            models => model_file.unwrap().models,
            model_files => self.model_files.model_files,
        )
        .into_bytes()
    }

    pub fn into_files(self) -> impl Iterator<Item = File> {
        let init_file = File {
            name: "models/__init__.py".into(),
            contents: self.init_file_contents(),
        };
        self.model_files
            .model_files
            .into_iter()
            .map(|(filename, model_file)| File {
                name: format!("models/{filename}.py"),
                contents: model_file.render().into_bytes(),
            })
            .chain(Some(init_file).into_iter())
    }
}

#[test]
fn model_file_generator_generate() {
    let mut generator = ModelFileGenerator::new("app", "public");
    let mut public = ModelFile::default();
    public.models.push(mock::user_model());
    generator.model_files.insert("public".into(), public);

    let expected = br#"from app.models.public import (
    User,
)"#;
    assert_eq!(generator.init_file_contents(), expected);
}
