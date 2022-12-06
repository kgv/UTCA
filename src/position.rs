use crate::app::App;
use anyhow::{bail, Error};
use std::fmt::{self, Display, Formatter};

/// Position
#[derive(Clone, Copy, Debug)]
pub enum Position {
    SnOneTwoThree,
    SnTwo,
    SnOneThree,
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::SnOneTwoThree => write!(f, "sn-1,2,3"),
            Self::SnTwo => write!(f, "sn-2"),
            Self::SnOneThree => write!(f, "sn-1,3"),
        }
    }
}

impl TryFrom<&str> for Position {
    type Error = Error;

    fn try_from(from: &str) -> Result<Self, Self::Error> {
        match from {
            "sn-123" => Ok(Self::SnOneTwoThree),
            "sn-13" => Ok(Self::SnOneThree),
            "sn-2" => Ok(Self::SnTwo),
            _ => bail!("Invalid `Euonymus` species"),
        }
    }
}
