use std::default;

use crate::{
    proto::{Catalog, Column, GenerateRequest, Identifier, Schema},
    query::Query,
    table::Table,
};

impl GenerateRequest {
    pub fn normalize_identifiers(&mut self) -> Option<()> {
        self.catalog.as_mut()?.normalize_identifiers();
        let schema = &self.catalog.as_ref()?.default_schema;
        self.queries
            .iter_mut()
            .for_each(|query| query.normalize_identifiers(schema));
        None
    }
}

impl Catalog {
    fn normalize_identifiers(&mut self) {
        let default_schema = self.default_schema.clone();

        for schema in &mut self.schemas {
            schema.normalize_identifiers(&default_schema)
        }
    }
}

impl Schema {
    fn normalize_identifiers(&mut self, default_schema: &str) {
        for table in &mut self.tables {
            table.normalize_identifiers(default_schema);
        }
    }
}

impl Table {
    pub fn normalize_identifiers(&mut self, default_schema: &str) -> Option<()> {
        self.rel
            .as_mut()
            .map(|ident| ident.normalize(default_schema));

        for column in &mut self.columns {
            column.normalize_identifiers(default_schema);
        }

        None
    }
}

impl Column {
    pub fn normalize_identifiers(&mut self, default_schema: &str) {
        self.embed_table
            .as_mut()
            .map(|ident| ident.normalize(default_schema));
        self.table
            .as_mut()
            .map(|ident| ident.normalize(default_schema));
        self.r#type
            .as_mut()
            .map(|ident| ident.normalize(default_schema));
    }
}

impl Identifier {
    pub fn normalize(&mut self, default_schema: &str) {
        if self.schema == "" {
            self.schema = default_schema.into();
        }
    }
}

impl Query {
    pub fn normalize_identifiers(&mut self, default_schema: &str) {
        for column in &mut self.columns {
            column.normalize_identifiers(default_schema);
        }
        self.columns
            .sort_by(|column1, column2| column1.name.cmp(&column2.name));

        self.insert_into_table
            .as_mut()
            .map(|ident| ident.normalize(default_schema));
    }
}
