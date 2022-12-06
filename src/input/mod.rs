use crate::{output::Output, Taxonomy, Triplet};
use indexmap::{
    map::{IntoIter, Iter, IterMut},
    IndexMap,
};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::{
    default::default,
    ops::{Deref, DerefMut},
};
pub use widgets::{ListWidget, TableWidget};

/// Input
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Input(IndexMap<Taxonomy, IndexMap<String, Vec<f64>>>);

impl Input {
    pub fn new(input: IndexMap<Taxonomy, IndexMap<String, Vec<f64>>>) -> Self {
        Self(input)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn list(&mut self) -> ListWidget {
        ListWidget {
            input: self.clone(),
            ..default()
        }
    }

    pub fn table(&self, inverted: bool) -> TableWidget {
        TableWidget {
            input: self.clone(),
            inverted,
            ..default()
        }
    }

    pub fn output(&self) -> Output {
        Output::new(
            self.0
                .iter()
                .map(|(key, value)| {
                    let value = (0..3)
                        .map(|_| value.keys())
                        .multi_cartesian_product()
                        .map(|key| {
                            let key = Triplet([key[0].clone(), key[1].clone(), key[2].clone()]);
                            let value =
                                value[&key[0]][0] * value[&key[1]][1] * value[&key[2]][0] * 0.0001;
                            (key, value)
                        })
                        .collect();
                    (*key, value)
                })
                .collect(),
        )
    }

    pub fn fatty_acids(&self) -> Vec<String> {
        self.0
            .values()
            .flat_map(IndexMap::keys)
            .unique()
            .cloned()
            .collect()
    }

    pub fn taxonomies(&self) -> Vec<Taxonomy> {
        self.0.keys().copied().collect()
    }
}

impl IntoIterator for Input {
    type Item = (Taxonomy, IndexMap<String, Vec<f64>>);

    type IntoIter = IntoIter<Taxonomy, IndexMap<String, Vec<f64>>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a Input {
    type Item = (&'a Taxonomy, &'a IndexMap<String, Vec<f64>>);

    type IntoIter = Iter<'a, Taxonomy, IndexMap<String, Vec<f64>>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a> IntoIterator for &'a mut Input {
    type Item = (&'a Taxonomy, &'a mut IndexMap<String, Vec<f64>>);

    type IntoIter = IterMut<'a, Taxonomy, IndexMap<String, Vec<f64>>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}

impl Deref for Input {
    type Target = IndexMap<Taxonomy, IndexMap<String, Vec<f64>>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Input {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub mod widgets;
