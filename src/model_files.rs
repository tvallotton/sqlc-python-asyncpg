use std::collections::BTreeMap;

use crate::{model::Model, model_file::ModelFile, options::Options, proto::Schema};

#[derive(Debug, Default)]
pub struct ModelFiles {
    pub options: Options,
    pub model_files: BTreeMap<String, ModelFile>,
}

impl ModelFiles {
    pub fn new(options: &Options) -> Self {
        Self {
            options: options.clone(),
            model_files: Default::default(),
        }
    }
    pub fn add_schema(&mut self, schema: &Schema) {
        self.model_files.insert(
            schema.name.clone(),
            ModelFile::from_schema(schema, &self.options),
        );
    }

    pub fn add_model(&mut self, filename: &str, model: Model) {
        if let Some(model_file) = self.model_files.get_mut(filename) {
            return model_file.add_model(model);
        }

        let mut file = ModelFile::default();
        file.add_model(model);
        self.model_files.insert(filename.into(), file);
    }
}
