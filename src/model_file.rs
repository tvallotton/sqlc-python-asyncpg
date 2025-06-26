#[cfg(test)]
use crate::mock;
use crate::{model::Model, options::Options, proto::Schema};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Default, Debug, serde::Serialize)]
pub struct ModelFile {
    pub models: Vec<Model>,
}

impl ModelFile {
    pub fn from_schema(schema: &Schema, options: &Options) -> Self {
        let mut model_file = ModelFile::default();
        for table in &schema.tables {
            model_file.add_model(Model::from_table(&table, options));
        }
        model_file
    }

    pub fn imports(&self) -> BTreeSet<&str> {
        self.models
            .iter()
            .flat_map(|model| model.imports())
<<<<<<< HEAD
            .chain(Some("import dataclasses"))
=======
>>>>>>> main
            .collect::<BTreeSet<&str>>()
    }

    pub fn add_model(&mut self, model: Model) {
        self.models.push(model);
        self.models
            .sort_by(|m1, m2| m2.python_type.annotation.cmp(&m1.python_type.annotation));
    }

    pub fn render(&self) -> String {
        let imports = self.imports();

        return minijinja::render!(include_str!("../templates/model_file.py.jinja2"), imports=> imports,models=> &self.models);
    }
}

#[test]
fn model_file_imports() {
    let model = Model::from_table(&mock::user_table(), &Options::default());
    let model_file = ModelFile {
        models: vec![model],
    };
    assert!(model_file.imports().contains("import uuid"));
}

#[test]
fn model_file_render() {
    let expected = r#"import dataclasses
import uuid


@dataclass.dataclass
class User:
    email: str
    id: uuid.UUID
"#;

    let model = Model::from_table(&mock::user_table(), &Options::default());
    let model_file = ModelFile {
        models: vec![model],
    };
    let contents = model_file.render();
    assert_eq!(contents, expected);
}

#[test]
fn model_file_render_two_tables() {
    let expected = r#"import dataclasses
import uuid


@dataclass.dataclass
class User:
    email: str
    id: uuid.UUID

@dataclass.dataclass
class UserReview:
    id: uuid.UUID
    url: str
"#;

    let model1 = Model::from_table(&mock::user_table(), &Options::default());
    let model2 = Model::from_table(&&mock::user_review_table(), &Options::default());
    let mut model_file = ModelFile::default();
    model_file.add_model(model1);
    model_file.add_model(model2);
    let contents = model_file.render();
    println!("{contents}");
    assert_eq!(contents, expected);
}
