use crate::{config::Sort, Taxonomy, Triplet};
use indexmap::{map::Iter, IndexMap};
use serde::{Deserialize, Serialize};
use std::{
    default::default,
    ops::{Bound, Deref},
};
pub use widgets::{ListWidget, PlotWidget, TableWidget};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Output(IndexMap<Taxonomy, IndexMap<Triplet, f64>>);

impl Output {
    pub fn new(output: IndexMap<Taxonomy, IndexMap<Triplet, f64>>) -> Self {
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
        for value in self.0.values_mut() {
            value.retain(|key, value| f(key, *value));
        }
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
}

impl Output {
    pub fn list(&self) -> ListWidget {
        ListWidget {
            data: self.clone(),
            ..default()
        }
    }

    pub fn table(&self, inverted: bool) -> TableWidget {
        TableWidget {
            data: self.0.clone(),
            inverted,
            ..default()
        }
    }

    pub fn plot(&self, inverted: bool) -> PlotWidget {
        PlotWidget {
            data: self.0.clone(),
            inverted,
            ..default()
        }
    }
}

impl<'a> IntoIterator for &'a Output {
    type Item = (&'a Taxonomy, &'a IndexMap<Triplet, f64>);

    type IntoIter = Iter<'a, Taxonomy, IndexMap<Triplet, f64>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

// impl Deref for Output {
//     type Target = IndexMap<Taxonomy, IndexMap<Triplet, f64>>;

//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

mod widgets;
