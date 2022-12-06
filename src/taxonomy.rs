use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};

/// Taxonomy
pub type Taxonomy = Euonymus;

/// Euonymus genus
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum Euonymus {
    Euonymus(subgenus::Euonymus),
    Kalonymus(subgenus::Kalonymus),
}

impl Display for Euonymus {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Euonymus(euonymus) if f.alternate() => write!(f, "Euonymus::{euonymus:#}"),
            Self::Euonymus(euonymus) => write!(f, "{euonymus}"),
            Self::Kalonymus(kalonymus) if f.alternate() => write!(f, "Euonymus::{kalonymus:#}"),
            Self::Kalonymus(kalonymus) => write!(f, "{kalonymus}"),
        }
    }
}

impl<'a> TryFrom<&[&'a str]> for Euonymus {
    type Error = Option<&'a str>;

    fn try_from(value: &[&'a str]) -> Result<Self, Self::Error> {
        match value.first() {
            Some(subgenus) => match &*subgenus.to_lowercase() {
                "euonymus" => Ok(Self::Euonymus(value[1..].try_into()?)),
                "kalonymus" => Ok(Self::Kalonymus(value[1..].try_into()?)),
                _ => Err(Some(subgenus)),
            },
            None => Err(None),
        }
    }
}

pub mod subgenus {
    use serde::{Deserialize, Serialize};
    use std::fmt::{self, Display, Formatter};

    /// Euonymus subgenus
    #[allow(clippy::enum_variant_names)]
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum Euonymus {
        Euonymus(super::sections::Euonymus),
        Melanocarya(super::sections::Melanocarya),
        Pseudovyenomus(super::sections::Pseudovyenomus),
    }

    impl Display for Euonymus {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            match self {
                Self::Euonymus(euonymus) if f.alternate() => write!(f, "Euonymus::{euonymus:#}"),
                Self::Melanocarya(melanocarya) if f.alternate() => {
                    write!(f, "Euonymus::{melanocarya:#}")
                }
                Self::Pseudovyenomus(pseudovyenomus) if f.alternate() => {
                    write!(f, "Euonymus::{pseudovyenomus:#}")
                }
                Self::Euonymus(euonymus) => write!(f, "{euonymus}"),
                Self::Melanocarya(melanocarya) => write!(f, "{melanocarya}"),
                Self::Pseudovyenomus(pseudovyenomus) => write!(f, "{pseudovyenomus}"),
            }
        }
    }

    impl<'a> TryFrom<&[&'a str]> for Euonymus {
        type Error = Option<&'a str>;

        fn try_from(value: &[&'a str]) -> Result<Self, Self::Error> {
            match value.first() {
                Some(section) => match &*section.to_lowercase() {
                    "euonymus" => Ok(Self::Euonymus(value[1..].try_into()?)),
                    "melanocarya" => Ok(Self::Melanocarya(value[1..].try_into()?)),
                    "pseudovyenomus" => Ok(Self::Pseudovyenomus(value[1..].try_into()?)),
                    _ => Err(Some(section)),
                },
                None => Err(None),
            }
        }
    }

    /// Kalonymus subgenus
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum Kalonymus {
        Latifolius,
        Macropterus,
        Maximowiczianus,
        Sachalinensis,
    }

    impl Display for Kalonymus {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            match self {
                Self::Latifolius if f.alternate() => write!(f, "Euonymus::Latifolius"),
                Self::Macropterus if f.alternate() => write!(f, "Euonymus::Macropterus"),
                Self::Maximowiczianus if f.alternate() => write!(f, "Euonymus::Maximowiczianus"),
                Self::Sachalinensis if f.alternate() => write!(f, "Euonymus::Sachalinensis"),
                Self::Latifolius => write!(f, "Latifolius"),
                Self::Macropterus => write!(f, "Macropterus"),
                Self::Maximowiczianus => write!(f, "Maximowiczianus"),
                Self::Sachalinensis => write!(f, "Sachalinensis"),
            }
        }
    }

    impl<'a> TryFrom<&[&'a str]> for Kalonymus {
        type Error = Option<&'a str>;

        fn try_from(value: &[&'a str]) -> Result<Self, Self::Error> {
            match value.first() {
                Some(specie) => match &*specie.to_lowercase() {
                    "latifolius" => Ok(Self::Latifolius),
                    "macropterus" => Ok(Self::Macropterus),
                    "maximowiczianus" => Ok(Self::Maximowiczianus),
                    "sachalinensis" => Ok(Self::Sachalinensis),
                    _ => Err(Some(specie)),
                },
                None => Err(None),
            }
        }
    }
}

pub mod sections {
    use serde::{Deserialize, Serialize};
    use std::fmt::{self, Display, Formatter};

    /// Euonymus section
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum Euonymus {
        Bungeanus,
        Europaeus,
        Hamiltonianus,
        Phellomanus,
        Semiexsertus,
        Sieboldianus,
    }

    impl Display for Euonymus {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            match self {
                Self::Bungeanus if f.alternate() => write!(f, "Euonymus::Bungeanus"),
                Self::Europaeus if f.alternate() => write!(f, "Euonymus::Europaeus"),
                Self::Hamiltonianus if f.alternate() => write!(f, "Euonymus::Hamiltonianus"),
                Self::Phellomanus if f.alternate() => write!(f, "Euonymus::Phellomanus"),
                Self::Semiexsertus if f.alternate() => write!(f, "Euonymus::Semiexsertus"),
                Self::Sieboldianus if f.alternate() => write!(f, "Euonymus::Sieboldianus"),
                Self::Bungeanus => write!(f, "Bungeanus"),
                Self::Europaeus => write!(f, "Europaeus"),
                Self::Hamiltonianus => write!(f, "Hamiltonianus"),
                Self::Phellomanus => write!(f, "Phellomanus"),
                Self::Semiexsertus => write!(f, "Semiexsertus"),
                Self::Sieboldianus => write!(f, "Sieboldianus"),
            }
        }
    }

    impl<'a> TryFrom<&[&'a str]> for Euonymus {
        type Error = Option<&'a str>;

        fn try_from(value: &[&'a str]) -> Result<Self, Self::Error> {
            match value.first() {
                Some(specie) => match &*specie.to_lowercase() {
                    "bungeanus" => Ok(Self::Bungeanus),
                    "europaeus" => Ok(Self::Europaeus),
                    "hamiltonianus" => Ok(Self::Hamiltonianus),
                    "phellomanus" => Ok(Self::Phellomanus),
                    "semiexsertus" => Ok(Self::Semiexsertus),
                    "sieboldianus" => Ok(Self::Sieboldianus),
                    _ => Err(Some(specie)),
                },
                None => Err(None),
            }
        }
    }

    /// Melanocarya section
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum Melanocarya {
        Alatus,
        Sacrosantcus,
    }

    impl Display for Melanocarya {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            match self {
                Self::Alatus if f.alternate() => write!(f, "Melanocarya::Alatus"),
                Self::Sacrosantcus if f.alternate() => write!(f, "Melanocarya::Sacrosantcus"),
                Self::Alatus => write!(f, "Alatus"),
                Self::Sacrosantcus => write!(f, "Sacrosantcus"),
            }
        }
    }

    impl<'a> TryFrom<&[&'a str]> for Melanocarya {
        type Error = Option<&'a str>;

        fn try_from(value: &[&'a str]) -> Result<Self, Self::Error> {
            match value.first() {
                Some(specie) => match &*specie.to_lowercase() {
                    "alatus" => Ok(Self::Alatus),
                    "sacrosantcus" => Ok(Self::Sacrosantcus),
                    _ => Err(Some(specie)),
                },
                None => Err(None),
            }
        }
    }

    /// Pseudovyenomus section
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum Pseudovyenomus {
        Pauciflorus,
    }

    impl Display for Pseudovyenomus {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            match self {
                Self::Pauciflorus if f.alternate() => write!(f, "Pseudovyenomus::Pauciflorus"),
                Self::Pauciflorus => write!(f, "Pauciflorus"),
            }
        }
    }

    // impl<'a> TryFrom<&[&'a str]> for Pseudovyenomus {
    //     type Error = Option<&'a str>;

    //     fn try_from(value: &[&'a str]) -> Result<Self, Self::Error> {
    //         match value.first() {
    //             Some(specie) => match &*specie.to_lowercase() {
    //                 "pauciflorus" => Ok(Self::Pauciflorus),
    //                 _ => Err(Some(specie)),
    //             },
    //             None => Err(None),
    //         }
    //     }
    // }

    impl<'a> TryFrom<&[&'a str]> for Pseudovyenomus {
        type Error = Option<&'a str>;

        fn try_from(value: &[&'a str]) -> Result<Self, Self::Error> {
            match value.first() {
                Some(specie) => match &*specie.to_lowercase() {
                    "pauciflorus" => Ok(Self::Pauciflorus),
                    _ => Err(Some(specie)),
                },
                None => Err(None),
            }
        }
    }
}
