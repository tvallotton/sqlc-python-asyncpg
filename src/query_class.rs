use std::collections::{BTreeMap, BTreeSet};

use crate::{
    method::{Method},
    proto::File,
    utils::to_pascal_case,
};

#[derive(Debug, serde::Serialize, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct QueryNamespace {
    name: String,
    subnamespaces: BTreeMap<String, QueryNamespace>,
    methods: BTreeSet<Method>,
}

impl QueryNamespace {
    pub fn root() -> QueryNamespace {
        QueryNamespace {
            name: "Queries".into(),
            subnamespaces: Default::default(),
            methods: BTreeSet::new(),
        }
    }

    pub fn resolve(&mut self, name: &str) -> &mut QueryNamespace {
        self._resolve(&name.split('.').collect::<Vec<_>>())
    }

    pub fn _resolve(&mut self, name: &[&str]) -> &mut QueryNamespace {
        if name.is_empty() {
            return self;
        }

        let entry = self.subnamespaces.entry(name[0].into());

        let namespace = entry.or_insert_with(|| QueryNamespace {
            name: to_pascal_case(name[0].into()) + "Queries",
            methods: Default::default(),
            subnamespaces: Default::default(),
        });

        return namespace._resolve(&name[1..]);
    }

    pub fn add_method(&mut self, method: Method) {
        self.methods.insert(method);
    }

    pub fn _imports<'a>(&'a self, imports: &mut BTreeSet<&'a str>) {
        for method in &self.methods {
            method.imports(imports);
        }

        for (_, namespace) in &self.subnamespaces {
            namespace._imports(imports);
        }
    }

    pub fn imports<'a>(&'a self) -> BTreeSet<&'a str> {
        let mut imports = BTreeSet::default();
        self._imports(&mut imports);
        imports.insert("import dataclasses");
        imports.insert("import asyncpg");
        return imports;
    }

    pub fn render(&self) -> File {
        File {
            name: "queries.py".into(),
            contents: minijinja::render!(
                include_str!("../templates/queries.py.jinja2"), namespace => self, imports => self.imports()
            )
            .into_bytes(),
        }
    }
}

impl Default for QueryNamespace {
    fn default() -> Self {
        Self::root()
    }
}

#[test]
fn resolve_namespace() {
    let mut namespace = QueryNamespace::root();

    let resolution = namespace.resolve("foo.bar");
    assert_eq!(resolution.name, "Bar");
}
