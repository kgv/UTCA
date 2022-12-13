use crate::{
    config::{Composition, Sort},
    Config, Specie, Tags,
};
use indexmap::{map::Iter, IndexMap};
use itertools::Itertools;
pub use list::List;
pub use plot::Plot;
use serde::{Deserialize, Serialize};
use std::ops::{Bound, Deref};
pub use table::Table;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Output(IndexMap<Specie, IndexMap<Tags, f64>>);

impl Output {
    pub fn new(output: IndexMap<Specie, IndexMap<Tags, f64>>) -> Self {
        Self(output)
    }

    pub fn bound(self, bound: Bound<f64>) -> Self {
        self.filter(|_, value| match bound {
            Bound::Included(bound) => value >= bound,
            Bound::Excluded(bound) => value > bound,
            Bound::Unbounded => true,
        })
    }

    pub fn configure(self, config: &Config) -> Output {
        self.bound(config.bound)
            .map(|tags| match config.composition {
                Some(Composition::PositionalSpecie) => tags
                    .into_iter()
                    .flat_map(|tag| [tag.clone(), tag.reverse()])
                    .collect(),
                _ => tags,
            })
            .filter(|tags, _| {
                for tag in tags {
                    if tag == config.pattern {
                        return true;
                    }
                }
                false
            })
            .sort(config.sort.unwrap_or_default())
    }

    pub fn filter<F: Fn(&Tags, f64) -> bool>(mut self, f: F) -> Self {
        self.0.retain(|_, value| {
            value.retain(|key, value| f(key, *value));
            !value.is_empty()
        });
        self
    }

    pub fn map<F: Fn(Tags) -> Tags>(self, f: F) -> Self {
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

    fn tags(&self) -> Vec<&Tags> {
        self.0.values().flat_map(IndexMap::keys).unique().collect()
    }
}

impl Deref for Output {
    type Target = IndexMap<Specie, IndexMap<Tags, f64>>;

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
    type Item = (&'a Specie, &'a IndexMap<Tags, f64>);

    type IntoIter = Iter<'a, Specie, IndexMap<Tags, f64>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

mod list;
mod plot;
mod table;
