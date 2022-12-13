use serde::{Deserialize, Serialize};
use std::{
    collections::{
        btree_set::{IntoIter, Iter},
        BTreeSet,
    },
    fmt::{self, Display, Formatter},
    ops::{Deref, DerefMut},
};

/// Tags
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Tags(BTreeSet<Tag>);

impl Tags {
    pub fn new(tags: BTreeSet<Tag>) -> Self {
        Self(tags)
    }
}

impl Deref for Tags {
    type Target = BTreeSet<Tag>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for Tags {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for (index, tag) in self.0.iter().enumerate() {
            if index != 0 {
                f.write_str(", ")?;
            }
            write!(f, "{tag}")?;
        }
        Ok(())
    }
}

impl FromIterator<Tag> for Tags {
    fn from_iter<T: IntoIterator<Item = Tag>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl IntoIterator for Tags {
    type Item = Tag;

    type IntoIter = IntoIter<Tag>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a Tags {
    type Item = &'a Tag;

    type IntoIter = Iter<'a, Tag>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

/// 1,2,3-Triacyl-sn-glycerol
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Tag {
    fatty_acids: [String; 3],
}

impl Tag {
    pub fn new(fatty_acids: [String; 3]) -> Self {
        Self { fatty_acids }
    }

    pub fn reverse(mut self) -> Self {
        self.fatty_acids.reverse();
        self
    }
}

impl Deref for Tag {
    type Target = [String; 3];

    fn deref(&self) -> &Self::Target {
        &self.fatty_acids
    }
}

impl Display for Tag {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}{}",
            self.fatty_acids[0], self.fatty_acids[1], self.fatty_acids[2]
        )?;
        Ok(())
    }
}

impl<'a> FromIterator<&'a String> for Tag {
    fn from_iter<T: IntoIterator<Item = &'a String>>(iter: T) -> Self {
        let mut into_iter = iter.into_iter();
        let one = into_iter.next().cloned().unwrap_or_default();
        let two = into_iter.next().cloned().unwrap_or_default();
        let three = into_iter.next().cloned().unwrap_or_default();
        Self {
            fatty_acids: [one, two, three],
        }
    }
}

impl PartialEq<Pattern> for &Tag {
    fn eq(&self, other: &Pattern) -> bool {
        for index in 0..3 {
            if let Some(fatty_acid) = &other.fatty_acids[index] {
                if fatty_acid != &self.fatty_acids[index] {
                    return false;
                }
            }
        }
        true
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Pattern {
    fatty_acids: [Option<String>; 3],
}

impl Deref for Pattern {
    type Target = [Option<String>; 3];

    fn deref(&self) -> &Self::Target {
        &self.fatty_acids
    }
}

impl DerefMut for Pattern {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.fatty_acids
    }
}

// impl From<&[&String]> for Tag {
//     fn from(value: &[&String]) -> Self {
//         let one = value.get(0).map_or_else(String::new, ToString::to_string);
//         let two = value.get(1).map_or_else(String::new, ToString::to_string);
//         let three = value.get(2).map_or_else(String::new, ToString::to_string);
//         Self {
//             fatty_acids: [one, two, three],
//         }
//     }
// }

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

// /// Triplet
// #[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
// pub struct Triplet(pub(crate) [String; 3]);

// impl Triplet {
//     pub fn normalize(mut self) -> Self {
//         if self.0[0] > self.0[2] {
//             self.0.reverse();
//         }
//         self
//     }
// }

// impl Deref for Triplet {
//     type Target = [String; 3];

//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

// impl Display for Triplet {
//     fn fmt(&self, f: &mut Formatter) -> fmt::Result {
//         write!(f, "{}{}{}", self.0[0], self.0[1], self.0[2])?;
//         if f.alternate() && self.0[0] != self.0[2] {
//             write!(f, ", {}{}{}", self.0[2], self.0[1], self.0[0])?;
//         }
//         Ok(())
//     }
// }
