use crate::{Composition, Output, Specie, Tag};
use indexmap::{
    map::{IntoIter, Iter, IterMut},
    IndexMap,
};
use itertools::Itertools;
pub use list::List;
use serde::{Deserialize, Serialize};
use std::{
    iter::once,
    ops::{Deref, DerefMut},
};
pub use text::Text;

/// Input
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Input(IndexMap<Specie, IndexMap<String, Vec<f64>>>);

impl Input {
    pub fn new(input: IndexMap<Specie, IndexMap<String, Vec<f64>>>) -> Self {
        Self(input)
    }

    pub fn output(&self) -> Output {
        Output::new(
            self.0
                .iter()
                .map(|(key, value)| {
                    let key = key.clone();
                    let value = (0..3)
                        .map(|_| value.keys())
                        .multi_cartesian_product()
                        .map(|key| {
                            let tag = Tag::new([key[0].clone(), key[1].clone(), key[2].clone()]);
                            let value =
                                value[&tag[0]][0] * value[&tag[1]][1] * value[&tag[2]][0] * 0.0001;
                            let composition = once(tag).collect();
                            (composition, value)
                        })
                        .collect();
                    (key, value)
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

    pub fn species(&self) -> Vec<Specie> {
        self.0.keys().cloned().collect()
    }
}

impl IntoIterator for Input {
    type Item = (Specie, IndexMap<String, Vec<f64>>);

    type IntoIter = IntoIter<Specie, IndexMap<String, Vec<f64>>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a Input {
    type Item = (&'a Specie, &'a IndexMap<String, Vec<f64>>);

    type IntoIter = Iter<'a, Specie, IndexMap<String, Vec<f64>>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a> IntoIterator for &'a mut Input {
    type Item = (&'a Specie, &'a mut IndexMap<String, Vec<f64>>);

    type IntoIter = IterMut<'a, Specie, IndexMap<String, Vec<f64>>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}

impl Deref for Input {
    type Target = IndexMap<Specie, IndexMap<String, Vec<f64>>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Input {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub mod list;
pub mod text;
