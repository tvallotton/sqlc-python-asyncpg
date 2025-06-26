use std::collections::BTreeMap;

use crate::{
    method::{MaybeEmbeddedType, Method, MethodOutput, MethodParameter},
    model::Model,
    model_file::ModelFile,
<<<<<<< HEAD
=======
    model_files::ModelFiles,
>>>>>>> main
    options::Options,
    proto::Identifier,
    python_type::PythonType,
    query::Query,
    query_class::QueryNamespace,
    utils::to_pascal_case,
};

<<<<<<< HEAD
pub struct MethodBuilder<'a> {
    pub options: &'a Options,
    pub model_files: &'a mut BTreeMap<String, ModelFile>,
}

impl<'a> MethodBuilder<'a> {
    pub fn new(model_files: &'a mut BTreeMap<String, ModelFile>, options: &'a Options) -> Self {
=======
pub struct MethodFactory<'a> {
    pub options: &'a Options,
    pub model_files: &'a mut ModelFiles,
}

impl<'a> MethodFactory<'a> {
    pub fn new(model_files: &'a mut ModelFiles, options: &'a Options) -> Self {
>>>>>>> main
        Self {
            model_files,
            options,
        }
    }

    pub fn build_method(&mut self, query: Query) -> Method {
        Method {
<<<<<<< HEAD
=======
            grouped_argument_type: self.build_grouped_argument_type(&query),
>>>>>>> main
            output: self.get_method_output(&query),
            parameters: MethodParameter::get_params_from_query(&query, self.options),
            query: query,
        }
    }

<<<<<<< HEAD
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

=======
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
>>>>>>> main
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
<<<<<<< HEAD
            python_type: query.model_type(self.options),
=======
            python_type: query.output_model_type(self.options),
>>>>>>> main
            fields,
        }
    }

<<<<<<< HEAD
    pub fn search_for_fitting_model(&self, query: &Query) -> Option<MethodOutput> {
=======
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
>>>>>>> main
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
<<<<<<< HEAD
=======
            .model_files
>>>>>>> main
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
<<<<<<< HEAD
    let builder = MethodBuilder::new(&mut files, &options);
    let output = builder.create_from_query(&&mock::query_get_all_posts());
=======
    let mut builder = MethodFactory::new(&mut files, &options);
    let output = builder.add_new_output_model_from_query(&&mock::query_get_all_posts());
>>>>>>> main
    dbg!(&output);
    assert_eq!(output.python_type.declaration.unwrap(), "GetAllPostsRow");
}

#[cfg(test)]
#[test]
fn search_for_fitting_model() {
    use crate::mock::{self, model_files};

    let mut files = model_files();
    let options = Default::default();
<<<<<<< HEAD
    let builder = MethodBuilder::new(&mut files, &options);

    let output = builder.search_for_fitting_model(&mock::query_fetch_user_by_id());
=======
    let builder = MethodFactory::new(&mut files, &options);

    let output = builder.search_for_existing_output_model(&mock::query_fetch_user_by_id());
>>>>>>> main

    assert_eq!(
        output.unwrap().python_type.declaration.as_deref(),
        Some("User")
    );
}
