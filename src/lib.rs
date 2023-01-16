use std::fmt;

use serde::{Deserialize, Serialize};

pub mod gen;
mod config;
pub use config::{Config, Generator};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Locale { 
    US,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Gender {
    Male,
    Female,
    Both,
}

impl fmt::Display for Locale {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Locale::US => write!(f, "us"),
        }
    }
}

impl fmt::Display for Gender {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Gender::Male => write!(f, "male"),
            Gender::Female => write!(f, "female"),
            Gender::Both => write!(f, "both")
        }
    }
}
