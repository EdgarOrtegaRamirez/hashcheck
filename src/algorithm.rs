use clap::ValueEnum;
use std::fmt;

#[derive(Clone, Debug, ValueEnum, PartialEq, Eq, Copy)]
pub enum Algorithm {
    Sha256,
    Sha512,
    Sha1,
    Md5,
    Blake2b,
}

impl fmt::Display for Algorithm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Algorithm::Sha256 => write!(f, "sha256"),
            Algorithm::Sha512 => write!(f, "sha512"),
            Algorithm::Sha1 => write!(f, "sha1"),
            Algorithm::Md5 => write!(f, "md5"),
            Algorithm::Blake2b => write!(f, "blake2b"),
        }
    }
}
