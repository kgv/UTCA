use serde::{Deserialize, Serialize};
use std::{
    fmt::{self, Display, Formatter},
    ops::Deref,
};

/// Triglyceride
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Triglyceride<T>([T; 3]);

impl<T: Display> Display for Triglyceride<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}{}{}", self.0[0], self.0[1], self.0[2])?;
        Ok(())
    }
}

// pub struct PositionalSpecies<T>(pub Triglyceride<T>);

// impl<T: Display> Display for PositionalSpecies<T> {
//     fn fmt(&self, f: &mut Formatter) -> fmt::Result {
//         write!(f, "{}", self.0)?;
//         if f.alternate() && self.0 .0[0] != self.0 .0[2] {
//             write!(f, ", {}{}{}", self.0[2], self.0[1], self.0[0])?;
//         }
//         Ok(())
//     }
// }

/// Triplet
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Triplet(pub(crate) [String; 3]);

impl Triplet {
    pub fn normalize(mut self) -> Self {
        if self.0[0] > self.0[2] {
            self.0.reverse();
        }
        self
    }
}

impl Deref for Triplet {
    type Target = [String; 3];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for Triplet {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}{}{}", self.0[0], self.0[1], self.0[2])?;
        if f.alternate() && self.0[0] != self.0[2] {
            write!(f, ", {}{}{}", self.0[2], self.0[1], self.0[0])?;
        }
        Ok(())
    }
}
