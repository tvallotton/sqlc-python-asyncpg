use std::collections::BTreeMap;

use crate::{
    method::{MaybeEmbeddedType, Method, MethodOutput, MethodParameter},
    model::Model,
    model_file::ModelFile,
    options::Options,
    proto::Identifier,
    python_type::PythonType,
    query::Query,
    query_class::QueryNamespace,
    utils::to_pascal_case,
};

pub struct MethodBuilder<'a> {
    pub options: &'a Options,
    pub model_files: &'a mut BTreeMap<String, ModelFile>,
}

impl<'a> MethodBuilder<'a> {
    pub fn new(model_files: &'a mut BTreeMap<String, ModelFile>, options: &'a Options) -> Self {
        Self {
            model_files,
            options,
        }
    }

    pub fn build_method(&mut self, query: Query) -> Method {
        Method {
            output: self.get_method_output(&query),
            parameters: MethodParameter::get_params_from_query(&query, self.options),
            query: query,
        }
    }

    pub fn get_method_output(&mut self, query: &Query) -> MethodOutput {
        if let Some(output) = self.search_for_fitting_model(query) {
            return output;
        }

        self.create_from_query(query)
    }

    pub fn create_from_query(&mut self, query: &Query) -> MethodOutput {
        let fields = query
            .columns
            .iter()
            .map(|column| {
                if let Some(table) = column.embed_table.as_ref() {
                    let model = self.get_model(&table).unwrap().clone();
                    return (column.name.clone(), model.python_type.clone());
                }
                (column.name.clone(), self.options.get_python_type(column))
            })
            .collect();

        let model = Model {
            table_name: None,
            python_type: query.model_type(self.options),
            fields,
        };
        match self.model_files.get_mut(query.module_name()) {
            Some(model_file) => {
                model_file.add_model(model);
            }
            None => {
                self.model_files.insert(
                    query.module_name().into(),
                    ModelFile {
                        models: vec![model],
                    },
                );
            }
        }

        let fields = query
            .columns
            .iter()
            .map(|column| {
                if let Some(table) = column.embed_table.as_ref() {
                    let model = self.get_model(&table).unwrap().clone();
                    return (column.name.clone(), MaybeEmbeddedType::Embedded(model));
                }
                (
                    column.name.clone(),
                    MaybeEmbeddedType::Simple(self.options.get_python_type(column)),
                )
            })
            .collect();

        MethodOutput {
            python_type: query.model_type(self.options),
            fields,
        }
    }

    pub fn search_for_fitting_model(&self, query: &Query) -> Option<MethodOutput> {
        let column = query.columns.get(0)?;

        let model = self.get_model(column.table.as_ref()?)?;

        let query_columns = query
            .columns
            .iter()
            .map(|col| (&col.name, self.options.get_python_type(col)));

        let model_columns = model
            .fields
            .iter()
            .map(|(name, type_)| (name, type_.clone()));
        dbg!();
        if Iterator::eq(query_columns, model_columns) {
            dbg!();
            return Some(MethodOutput::from_model(model));
        }

        dbg!();
        None
    }

    pub fn get_model(&self, ident: &Identifier) -> Option<&Model> {
        self.model_files
            .get(&ident.schema)?
            .models
            .iter()
            .find(|model| model.table_name.as_deref() == Some(&ident.name))
    }
}
#[cfg(test)]
#[test]
fn create_from_query() {
    use crate::mock::{self, model_files};

    let mut files = model_files();
    let options = Default::default();
    let builder = MethodBuilder::new(&mut files, &options);
    let output = builder.create_from_query(&&mock::query_get_all_posts());
    dbg!(&output);
    assert_eq!(output.python_type.declaration.unwrap(), "GetAllPostsRow");
}

#[cfg(test)]
#[test]
fn search_for_fitting_model() {
    use crate::mock::{self, model_files};

    let mut files = model_files();
    let options = Default::default();
    let builder = MethodBuilder::new(&mut files, &options);

    let output = builder.search_for_fitting_model(&mock::query_fetch_user_by_id());

    assert_eq!(
        output.unwrap().python_type.declaration.as_deref(),
        Some("User")
    );
}
