use crate::{
    config::{PositionalComposition, Sort},
    Composition, Specie,
};
use indexmap::{
    map::{IntoIter, Iter, IterMut},
    IndexMap,
};
use itertools::Itertools;
pub use list::List;
pub use plot::Plot;
use serde::{Deserialize, Serialize};
use std::{
    default::default,
    ops::{Bound, Deref, Index},
};
pub use table::Table;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Output(IndexMap<Specie, IndexMap<Composition, f64>>);

impl Output {
    pub fn new(output: IndexMap<Specie, IndexMap<Composition, f64>>) -> Self {
        Self(output)
    }

    pub fn bound(self, bound: Bound<f64>) -> Self {
        self.filter(|_, value| match bound {
            Bound::Included(bound) => value >= bound,
            Bound::Excluded(bound) => value > bound,
            Bound::Unbounded => true,
        })
    }

    pub fn filter<F: Fn(&Composition, f64) -> bool>(mut self, f: F) -> Self {
        self.0.retain(|_, value| {
            value.retain(|key, value| f(key, *value));
            !value.is_empty()
        });
        self
    }

    pub fn map<F: Fn(Composition) -> Composition>(self, f: F) -> Self {
        Output(
            self.0
                .into_iter()
                .map(move |(key, value)| {
                    let value = value
                        .into_iter()
                        .fold(IndexMap::new(), |mut map, (key, value)| {
                            map.entry(f(key))
                                .and_modify(|accumulator| *accumulator += value)
                                .or_insert(value);
                            map
                        });
                    (key, value)
                })
                .collect(),
        )
    }

    pub fn sort(mut self, sort: Sort) -> Self {
        for value in self.0.values_mut() {
            match sort {
                Sort::Key => value.sort_keys(),
                Sort::Value => value.sort_by(|_, a, _, b| a.total_cmp(b)),
            }
        }
        self
    }

    fn species(&self) -> Vec<&Specie> {
        self.0.keys().collect()
    }

    fn compositions(&self) -> Vec<&Composition> {
        self.0.values().flat_map(IndexMap::keys).unique().collect()
    }
}

impl Deref for Output {
    type Target = IndexMap<Specie, IndexMap<Composition, f64>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// impl IntoIterator for Output {
//     type Item = (Specie, IndexMap<Tags, f64>);

//     type IntoIter = IntoIter<Specie, IndexMap<Tags, f64>>;

//     fn into_iter(self) -> Self::IntoIter {
//         self.0.into_iter()
//     }
// }

// impl<'a> IntoIterator for &'a mut Output {
//     type Item = (&'a Specie, &'a mut IndexMap<Tags, f64>);

//     type IntoIter = IterMut<'a, Specie, IndexMap<Tags, f64>>;

//     fn into_iter(self) -> Self::IntoIter {
//         self.0.iter_mut()
//     }
// }

impl<'a> IntoIterator for &'a Output {
    type Item = (&'a Specie, &'a IndexMap<Composition, f64>);

    type IntoIter = Iter<'a, Specie, IndexMap<Composition, f64>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

mod list;
mod plot;
mod table;
