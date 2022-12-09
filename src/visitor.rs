use crate::taxonomy::Specie;
use anyhow::Error;
use indexmap::IndexMap;
use inflector::Inflector;
use toml_edit::{visit::*, Document, Item};

/// Collect the names of every dependency key.
#[derive(Debug, Default)]
pub struct Visitor<'a> {
    taxonomy: Vec<&'a str>,
    result: IndexMap<Specie, IndexMap<String, Vec<f64>>>,
    errors: Vec<Error>
}

impl Visitor<'_> {
    pub fn visit(document: &Document) -> IndexMap<Specie, IndexMap<String, Vec<f64>>> {
        let mut visitor = Visitor::default();
        visitor.visit_document(document);
        visitor.result
    }
}

impl<'a> Visit<'a> for Visitor<'a> {
    fn visit_table_like_kv(&mut self, key: &'a str, node: &'a Item) {
        if let Some(array) = node.as_array() {
            let value = array
                .iter()
                .map(|value| {
                    value
                        .as_float()
                        .unwrap_or_else(|| panic!("Parse value ({value}) as float"))
                })
                .collect();
            let specie = self
                .taxonomy
                .iter()
                .copied()
                .map(Inflector::to_title_case)
                .collect();
            self.result
                .entry(specie)
                .or_default()
                .insert(key.to_string(), value);
        } else {
            self.taxonomy.push(key);
            self.visit_item(node);
            self.taxonomy.pop();
        }
    }
}
