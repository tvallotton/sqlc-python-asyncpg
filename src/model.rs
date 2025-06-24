use minijinja::filters::map;

#[cfg(test)]
use crate::mock;
use crate::{
    options::Options,
    python_type::PythonType,
    query::{self, Query},
    table::Table,
    utils::to_pascal_case,
};
use std::collections::BTreeMap;

#[derive(Clone, serde::Serialize, PartialEq, PartialOrd)]
pub struct Model {
    table_name: Option<String>,
    pub python_type: PythonType,
    fields: BTreeMap<String, PythonType>,
}

impl Model {
    pub fn from_table(table: &Table, options: &Options) -> Self {
        let fields = table
            .columns
            .iter()
            .map(|column| (column.name.clone(), options.get_python_type(column)))
            .collect();
        Model {
            table_name: table.rel.as_ref().map(|rel| rel.name.clone()),
            fields,
            python_type: table.model_type(),
        }
    }

    pub fn from_query(query: Query, options: &Options) -> Self {
        todo!()
    }

    pub fn imports(&self) -> impl Iterator<Item = &str> {
        self.fields
            .iter()
            .filter_map(|(_, type_)| type_.import.as_deref())
    }
}

#[test]
pub fn model_from_table() {
    let options = Options::default();
    let user_table = mock::user_table();
    let model = Model::from_table(&user_table, &options);
    assert_eq!(model.table_name.unwrap(), "user");
    assert_eq!(model.python_type.import.unwrap(), "import models.public");
    assert_eq!(model.python_type.constructor, "models.public.User");
    assert_eq!(model.python_type.declaration.as_deref(), Some("User"));
}

#[test]
pub fn user_review_table() {
    let options = Options::default();
    let user_table = mock::user_review_table();
    let model = Model::from_table(&user_table, &options);
    assert_eq!(model.table_name.unwrap(), "user_review");
    assert_eq!(model.python_type.import.unwrap(), "import models.post");
    assert_eq!(model.python_type.constructor, "models.post.UserReview");
    assert_eq!(model.python_type.declaration.as_deref(), Some("UserReview"));
}

// #[test]
// pub fn model_from_query() {
//     let options = Options::default();
//     let query = Model::from_query(query, &options)
// }
