use crate::{
    config::{PositionalComposition, Sort},
    taxonomy::Specie,
    Triplet,
};
use indexmap::{
    map::{IntoIter, Iter, IterMut},
    IndexMap,
};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::{
    default::default,
    ops::{Bound, Deref, Index},
};
pub use widgets::{ListWidget, PlotWidget, TableWidget};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Output(IndexMap<Specie, IndexMap<Triplet, f64>>);

impl Output {
    pub fn new(output: IndexMap<Specie, IndexMap<Triplet, f64>>) -> Self {
        Self(output)
    }

    pub fn bounded(self, bound: Bound<f64>) -> Self {
        self.filter(|_, value| match bound {
            Bound::Included(bound) => value >= bound,
            Bound::Excluded(bound) => value > bound,
            Bound::Unbounded => true,
        })
    }

    pub fn filter<F: Fn(&Triplet, f64) -> bool>(mut self, f: F) -> Self {
        self.0.retain(|_, value| {
            value.retain(|key, value| f(key, *value));
            !value.is_empty()
        });
        self
    }

    pub fn map<F: Fn(Triplet) -> Triplet>(self, f: F) -> Self {
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

    pub fn positional_composition(
        self,
        positional_composition: Option<PositionalComposition>,
    ) -> Self {
        match positional_composition {
            Some(PositionalComposition::Specie) => self.map(Triplet::normalize),
            Some(PositionalComposition::Type) => self.map(Triplet::normalize),
            None => self,
        }
    }

    pub fn positional_species(self) -> Self {
        self.map(Triplet::normalize)
    }

    pub fn positional_type(self) -> Self {
        self.map(Triplet::normalize)
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

    fn triplets(&self) -> Vec<&Triplet> {
        self.0.values().flat_map(IndexMap::keys).unique().collect()
    }
}

impl Deref for Output {
    type Target = IndexMap<Specie, IndexMap<Triplet, f64>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// impl IntoIterator for Output {
//     type Item = (Specie, IndexMap<Triplet, f64>);

//     type IntoIter = IntoIter<Specie, IndexMap<Triplet, f64>>;

//     fn into_iter(self) -> Self::IntoIter {
//         self.0.into_iter()
//     }
// }

// impl<'a> IntoIterator for &'a mut Output {
//     type Item = (&'a Specie, &'a mut IndexMap<Triplet, f64>);

//     type IntoIter = IterMut<'a, Specie, IndexMap<Triplet, f64>>;

//     fn into_iter(self) -> Self::IntoIter {
//         self.0.iter_mut()
//     }
// }

impl<'a> IntoIterator for &'a Output {
    type Item = (&'a Specie, &'a IndexMap<Triplet, f64>);

    type IntoIter = Iter<'a, Specie, IndexMap<Triplet, f64>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

mod widgets;
