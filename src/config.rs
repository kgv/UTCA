use crate::tag::Pattern;
use serde::{Deserialize, Serialize};
use std::{
    default::default,
    fmt::{self, Display, Formatter},
    ops::Bound,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    pub bound: Bound<f64>,
    pub composition: Option<Composition>,
    pub pattern: Pattern,
    pub sort: Option<Sort>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            bound: Bound::Unbounded,
            composition: default(),
            pattern: default(),
            sort: default(),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub enum Io {
    Output,
    #[default]
    Input,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub enum InputView {
    #[default]
    List,
    Text,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub enum Sort {
    #[default]
    Key,
    Value,
}

impl Display for Sort {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Sort::Key => write!(f, "Key"),
            Sort::Value => write!(f, "Value"),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub enum OutputView {
    #[default]
    List,
    Plot,
    Table,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Composition {
    PositionalSpecie,
    PositionalType,
}

impl Composition {
    pub fn name(positional_composition: &Option<Self>) -> &'static str {
        match positional_composition {
            None => "",
            Some(Composition::PositionalSpecie) => "Positional-specie composition",
            Some(Composition::PositionalType) => "Positional-type composition",
        }
    }

    pub fn abbreviation(positional_composition: &Option<Self>) -> &'static str {
        match positional_composition {
            None => "",
            Some(Composition::PositionalSpecie) => "PSC",
            Some(Composition::PositionalType) => "PTC",
        }
    }
}
