use std::fmt::{self, Display, Formatter};

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub enum Io {
    Output,
    #[default]
    Input,
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
pub enum Widget {
    #[default]
    List,
    Plot,
    Table,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum PositionalComposition {
    Specie,
    Type,
}

impl PositionalComposition {
    pub fn name(positional_composition: &Option<Self>) -> &'static str {
        match positional_composition {
            None => "",
            Some(PositionalComposition::Specie) => "Positional-specie composition",
            Some(PositionalComposition::Type) => "Positional-type composition",
        }
    }

    pub fn abbreviation(positional_composition: &Option<Self>) -> &'static str {
        match positional_composition {
            None => "",
            Some(PositionalComposition::Specie) => "PSC",
            Some(PositionalComposition::Type) => "PTC",
        }
    }
}
