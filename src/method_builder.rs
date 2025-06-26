use std::collections::BTreeMap;

use crate::{
    method::{MaybeEmbeddedType, Method, MethodOutput, MethodParameter},
    model::Model,
    model_file::ModelFile,
    model_files::ModelFiles,
    options::Options,
    proto::Identifier,
    python_type::PythonType,
    query::Query,
    query_class::QueryNamespace,
    utils::to_pascal_case,
};

pub struct MethodFactory<'a> {
    pub options: &'a Options,
    pub model_files: &'a mut ModelFiles,
}

impl<'a> MethodFactory<'a> {
    pub fn new(model_files: &'a mut ModelFiles, options: &'a Options) -> Self {
        Self {
            model_files,
            options,
        }
    }

    pub fn build_method(&mut self, query: Query) -> Method {
        Method {
            grouped_argument_type: self.build_grouped_argument_type(&query),
            output: self.get_method_output(&query),
            parameters: MethodParameter::get_params_from_query(&query, self.options),
            query: query,
        }
    }

    pub fn build_grouped_argument_type(&mut self, query: &Query) -> Option<PythonType> {
        let python_type = query.input_model_type(&self.options)?;

        let fields = query
            .params
            .iter()
            .filter_map(|param| param.column.as_ref())
            .map(|column| (column.name.clone(), self.options.get_python_type(column)))
            .collect();

        let mut model = Model {
            table_name: None,
            python_type: python_type.clone(),
            fields,
            protocol_import: Some("import typing"),
        };

        if let Some("dataclass") = query.group_arguments() {
            model.protocol_import = None;
        }

        self.model_files.add_model(&query.namespace_name(), model);

        Some(python_type)
    }

    pub fn get_method_output(&mut self, query: &Query) -> MethodOutput {
        if let Some(output) = self.search_for_existing_output_model(query) {
            return output;
        }

        self.add_new_output_model_from_query(query)
    }

    pub fn add_new_output_model_from_query(&mut self, query: &Query) -> MethodOutput {
        self.add_output_model_for_query(query);
        return self.build_method_outputfor_query(query);
    }

    pub fn build_method_outputfor_query(&mut self, query: &Query) -> MethodOutput {
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
            python_type: query.output_model_type(self.options),
            fields,
        }
    }

    pub fn add_output_model_for_query(&mut self, query: &Query) {
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
            python_type: query.output_model_type(self.options),
            fields,
            protocol_import: None,
        };
        self.model_files.add_model(&query.module_name(), model);
    }

    pub fn search_for_existing_output_model(&self, query: &Query) -> Option<MethodOutput> {
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
            .model_files
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
    let mut builder = MethodFactory::new(&mut files, &options);
    let output = builder.add_new_output_model_from_query(&&mock::query_get_all_posts());
    dbg!(&output);
    assert_eq!(output.python_type.declaration.unwrap(), "GetAllPostsRow");
}

#[cfg(test)]
#[test]
fn search_for_fitting_model() {
    use crate::mock::{self, model_files};

    let mut files = model_files();
    let options = Default::default();
    let builder = MethodFactory::new(&mut files, &options);

    let output = builder.search_for_existing_output_model(&mock::query_fetch_user_by_id());

    assert_eq!(
        output.unwrap().python_type.declaration.as_deref(),
        Some("User")
    );
}
