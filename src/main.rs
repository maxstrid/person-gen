use std::error::Error;

use person_gen::gen::{Person, ToResult};
use person_gen::Config;

use directories::BaseDirs;

#[cfg(target_family = "windows")]
const CONFIG_FILE: &str = r#"\Config.toml"#;

#[cfg(target_family = "unix")]
const CONFIG_FILE: &str = "/config.toml";

fn main() {
    let config = match get_config() {
       Ok(config) => config,
       Err(_) => Config::default(),
    };

    let locale = config.generator.locale;
    let gender = config.generator.gender;

    let person = Person::new(locale, gender).unwrap_or_else(|e| {
        println!("{e}");
        std::process::exit(1);
    });

    let first_name = person.generate_first_name();
    let last_name = person.generate_last_name();

    println!("{first_name} {last_name}");
}

pub fn get_config() -> Result<Config, Box<dyn Error>> {
    let base_dir = BaseDirs::new().to_result()?;
    let config_dir = base_dir.config_dir().to_str().to_result()?; 

    let config_file = format!("{config_dir}{CONFIG_FILE}");

    Ok(Config::read(config_file)?)
}
