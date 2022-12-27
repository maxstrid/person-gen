use std::error::Error;
use std::path::Path;
use std::fmt;
use std::fs;

use directories::ProjectDirs;

use rand::seq::SliceRandom;

mod error;
use error::ToResult;

#[cfg(target_family = "windows")]
const RESOURCES: &str = r#"\Resources\"#;

#[cfg(target_family = "unix")]
const RESOURCES: &str = "/resources/";

#[derive(Debug, Clone, Copy)]
pub enum Locale {
    USEN,
}

#[derive(Debug, Clone, Copy)]
pub enum Gender {
    Male,
    Female,
}

impl fmt::Display for Locale {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Locale::USEN => write!(f, "en_US"),
        }
    }
}

impl fmt::Display for Gender {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Gender::Male => write!(f, "male"),
            Gender::Female => write!(f, "female"),
        }
    }
}

pub struct Person {
    first_names: Vec<String>,
    last_names: Vec<String>,
}

impl Person {
    pub fn new(locale: Locale, gender: Gender) -> Result<Self, Box<dyn Error>> {
        let gender = gender.to_string();
        let first_names = Self::read(locale, format!("{gender}_firstnames"))?;
        let last_names = Self::read(locale, String::from("last_names"))?;
        
        Ok(Self {
            first_names,
            last_names,
        })
    }

    pub fn generate_first_name(&self) -> String {
       Self::get_random(&self.first_names)  
    }

    pub fn generate_last_name(&self) -> String {
        Self::get_random(&self.last_names)
    }

    fn get_random(vec: &Vec<String>) -> String {
        let mut rng = rand::thread_rng();
        vec.choose(&mut rng).unwrap().clone()
    }

    fn read(locale: Locale, name: String) -> Result<Vec<String>, Box<dyn Error>> {
        let project_dir = ProjectDirs::from("io.github.maxstrid", "Person Gen", "Person Gen").to_result()?;
        let data_dir = project_dir.data_dir().to_str().to_result()?;

        let resource_dir = format!("{data_dir}{RESOURCES}");

        let locale = locale.to_string(); 
        let item = format!("{resource_dir}{locale}.{name}.txt"); 

        if !Path::new(&item).exists() {
            return Err(Box::new(error::Error::UnknownLocaleError))
        }

        let item = fs::read_to_string(&item)?;

        let item = item
                   .lines()
                   .filter(|item| !item.contains("#") && !item.is_empty())
                   .map(|s| s.trim().to_string())
                   .collect::<Vec<String>>();
        Ok(item)
    }
}
