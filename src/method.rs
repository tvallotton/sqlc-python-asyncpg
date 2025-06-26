use std::collections::{BTreeMap, BTreeSet};

use minijinja::render;

#[cfg(test)]
use crate::mock::{query_fetch_user_by_id, user_model};

use crate::{
    model::Model, options::Options, proto::Parameter, python_type::PythonType, query::Query,
};

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, PartialOrd, Ord)]
pub struct Method {
    pub query: Query,
    pub parameters: Vec<MethodParameter>,
    pub output: MethodOutput,
    pub grouped_argument_type: Option<PythonType>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, PartialOrd, Ord)]
pub struct MethodOutput {
    pub python_type: PythonType,
    pub fields: BTreeMap<String, MaybeEmbeddedType>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, PartialOrd, Ord)]
pub struct MethodParameter {
    name: String,
    python_type: Option<PythonType>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, PartialOrd, Ord)]
#[serde(tag = "type")]
pub enum MaybeEmbeddedType {
    Simple(PythonType),
    Embedded(Model),
}

impl MethodParameter {
    pub fn imports(&self) -> impl Iterator<Item = &str> {
        self.python_type
            .as_ref()
            .into_iter()
            .filter_map(|type_| type_.import.as_deref())
    }

    fn from_query_parameter(param: &Parameter, options: &Options) -> Self {
        if let Some(column) = param.column.as_ref() {
            return Self {
                name: column.name.clone(),
                python_type: param
                    .column
                    .as_ref()
                    .map(|column| options.get_python_type(column)),
            };
        }

        Self {
            name: format!("p{}", param.number),
            python_type: None,
        }
    }

    pub fn get_params_from_query(query: &Query, options: &Options) -> Vec<Self> {
        query
            .params
            .iter()
            .map(|param| Self::from_query_parameter(param, options))
            .collect()
    }
}

impl MethodOutput {
    pub fn from_model(model: &Model) -> Self {
        let fields = model
            .fields
            .iter()
            .map(|(name, type_)| (name.clone(), MaybeEmbeddedType::Simple(type_.clone())))
            .collect();
        MethodOutput {
            python_type: model.python_type.clone(),
            fields,
        }
    }

    pub fn imports<'a>(&'a self, imports: &mut BTreeSet<&'a str>) {
        imports.extend(self.python_type.import.as_deref());

        for type_ in self.fields.values() {
            match type_ {
                MaybeEmbeddedType::Simple(field) => imports.extend(field.import.as_deref()),
                MaybeEmbeddedType::Embedded(model) => imports.extend(model.imports()),
            }
        }
    }
}

impl Method {
    pub fn render(&self) -> String {
        minijinja::render!(include_str!("../templates/method.py.jinja2"), method=> self)
    }

    pub fn imports<'a>(&'a self, imports: &mut BTreeSet<&'a str>) {
        self.output.imports(imports);
        for param in &self.parameters {
            imports.extend(param.imports())
        }
    }
}

#[cfg(test)]
#[test]
fn method_params_from_query() {
    use crate::mock;

    let param = MethodParameter::from_query_parameter(
        &mock::query_fetch_user_by_id().params[0],
        &Default::default(),
    );
    assert_eq!(param.name, "id");
}

#[test]
fn method_output_from_model() {
    let method_output = MethodOutput::from_model(&user_model());
}

#[test]
fn render_method() {
    let method = Method {
        query: query_fetch_user_by_id(),
        output: MethodOutput::from_model(&user_model()),
        parameters: MethodParameter::get_params_from_query(
            &query_fetch_user_by_id(),
            &Default::default(),
        ),
    };
    let expected = r#"async def fetch_user_by_id(self, id: uuid.UUID) -> models.public.User:
    row = await self.connection.fetchrow(
        self.FETCH_USER_BY_ID, id
    )
    return models.public.User(
        email=row["email"],
        id=row["id"],
    )"#;

    assert_eq!(method.render(), expected);
}

#[test]
fn render_method_with_embedded() {}
