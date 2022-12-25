use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};

/// Specie
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Specie {
    taxonomy: Vec<String>,
}

impl Specie {
    pub fn new() -> Self {
        Self {
            taxonomy: Vec::new(),
        }
    }

    pub fn taxonomy(&self, sep: &str) -> String {
        self.taxonomy.join(sep)
    }
}

impl Display for Specie {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if let Some(specie) = self.taxonomy.last() {
            write!(f, "{specie}")?;
        }
        Ok(())
    }
}

impl From<Vec<String>> for Specie {
    fn from(taxonomy: Vec<String>) -> Self {
        Self { taxonomy }
    }
}

impl FromIterator<String> for Specie {
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
        Self {
            taxonomy: iter.into_iter().collect(),
        }
    }
}
