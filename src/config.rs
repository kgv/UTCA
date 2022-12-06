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

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub enum Widget {
    #[default]
    List,
    Plot,
    Table,
}

// #[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
// struct Compositions {
//     psc: bool,
//     ptc: bool,
// }

// #[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
// enum Composition {
//     PositionalSpecies,
//     PositionalType,
// }
