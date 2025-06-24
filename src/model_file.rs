#[cfg(test)]
use crate::mock;
use crate::{model::Model, options::Options};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Default)]
pub struct ModelFile {
    pub models: Vec<Model>,
}

impl ModelFile {
    pub fn imports(&self) -> impl Iterator<Item = &str> {
        self.models.iter().flat_map(|model| model.imports())
    }

    pub fn render_file(&self) -> String {
        todo!()
    }
}

#[test]
fn model_file_imports() {
    let model = Model::from_table(&mock::user_table(), &Options::default());
    let model_file = ModelFile {
        models: vec![model],
    };
    assert!(model_file.imports().any(|import| import == "import uuid"));
}

#[test]
fn model_file_render() {
    let model = Model::from_table(&mock::user_table(), &Options::default());
    let model_file = ModelFile {
        models: vec![model],
    };
    let contents = model_file.render_file();
}
