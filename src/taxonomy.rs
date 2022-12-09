use std::borrow::Borrow;

/// Specie
pub type Specie = Vec<String>;

/// Taxonomy
pub trait Taxonomy {
    fn taxonomy(&self, sep: &str) -> String;

    fn name(&self) -> &str;
}

impl<T: Borrow<str>> Taxonomy for [T] {
    fn taxonomy(&self, sep: &str) -> String {
        self.join(sep)
    }

    fn name(&self) -> &str {
        self.last().map_or("", Borrow::borrow)
    }
}
