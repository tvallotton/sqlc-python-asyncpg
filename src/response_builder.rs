
use crate::{
    method_builder::MethodFactory,
    model_file_generator::ModelFileGenerator,
    model_files::ModelFiles,
    options::Options,
    proto::{File, GenerateRequest, GenerateResponse},
    query_class::QueryNamespace,
};

#[derive(Default)]
pub struct ResponseBuilder {
    root: QueryNamespace,
    model_files: ModelFiles,
    request: GenerateRequest,
    options: Options,
}

impl ResponseBuilder {
    pub fn new(request: GenerateRequest) -> Self {
        let options = Options::from_request(&request);
        Self {
            root: QueryNamespace::root(),
            model_files: ModelFiles::new(&options),
            options: Options::from_request(&request),
            request,
        }
    }

    pub fn add_catalog(&mut self) -> Option<()> {
        for schema in &self.request.catalog.as_ref()?.schemas {
            self.model_files.add_schema(schema);
        }
        None
    }

    pub fn add_queries(&mut self) {
        for query in &self.request.queries {
            let mut builder = MethodFactory {
                model_files: &mut self.model_files,
                options: &self.options,
            };
            let method = builder.build_method(query.clone());

            self.root
                .resolve(&query.namespace_name())
                .add_method(method);
        }
    }

    pub fn build(mut self) -> GenerateResponse {
        self.add_catalog();
        self.add_queries();
        let mut reponse = GenerateResponse::default();

        reponse.files.push(self.root.render());

        reponse.files.extend(self.model_files());

        reponse
    }

    pub fn model_files(self) -> impl Iterator<Item = File> {
        let generator = ModelFileGenerator {
            package: self.options.package,
            default_schema: self.request.catalog.unwrap().default_schema,
            model_files: self.model_files,
        };

        generator.into_files()
    }
}
