// Copyright (c) Microsoft. All rights reserved.

use std::fmt;
use std::str::FromStr;

pub const API_VERSION: Version = Version::Version2019_01_30;

#[derive(Clone, Copy, Debug, PartialOrd, PartialEq)]
pub enum Version {
    Version2018_06_28,
    Version2019_01_30,
}

impl FromStr for Version {
    type Err = ();

    fn from_str(s: &str) -> Result<Version, ()> {
        match s {
            "2018-06-28" => Ok(Version::Version2018_06_28),
            "2019-01-30" => Ok(Version::Version2019_01_30),
            _ => Err(()),
        }
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Version::Version2018_06_28 => write!(f, "2018-06-28"),
            Version::Version2019_01_30 => write!(f, "2019-01-30"),
        }
    }
}
