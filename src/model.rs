use minijinja::filters::map;

#[cfg(test)]
use crate::mock;
use crate::{
    options::Options,
    proto::Column,
    python_type::PythonType,
    query::{self, Query},
    table::Table,
    utils::to_pascal_case,
};
use std::collections::BTreeMap;

#[derive(Clone, Debug, serde::Serialize, PartialEq, PartialOrd, Ord, Eq)]
pub struct Model {
    pub table_name: Option<String>,
    pub python_type: PythonType,
    pub fields: BTreeMap<String, PythonType>,
    pub protocol_import: Option<&'static str>,
}

impl Model {
    pub fn from_table(table: &Table, options: &Options) -> Self {
        Model {
            table_name: table.rel.as_ref().map(|rel| rel.name.clone()),
            fields: Self::column_to_fields(&table.columns, options),
            python_type: table.model_type(options),
            protocol_import: None,
        }
    }

    pub fn from_query(query: &Query, options: &Options) -> Self {
        Model {
            table_name: None,
            fields: Self::column_to_fields(&query.columns, options),
            python_type: query.output_model_type(options),
            protocol_import: None,
        }
    }

    pub fn imports(&self) -> impl Iterator<Item = &str> {
        self.fields
            .iter()
            .filter_map(|(_, type_)| type_.import.as_deref())
            .chain(
                self.protocol_import
                    .or(Some("import dataclasses"))
                    .into_iter(),
            )
    }

    pub fn column_to_fields(columns: &[Column], options: &Options) -> BTreeMap<String, PythonType> {
        columns
            .iter()
            .map(|column| (column.name.clone(), options.get_python_type(column)))
            .collect()
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

#[test]
pub fn model_from_query() {
    let options = Options::default();
    let model = Model::from_query(&mock::query_fetch_user_by_id(), &options);

    assert_eq!(
        model.python_type.declaration.as_deref(),
        Some("FetchUserByIdRow")
    );
    model.fields.get("email").unwrap();
}
