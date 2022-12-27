use std::fs;
use std::env;
use std::path::Path;

use directories::ProjectDirs;

#[cfg(target_family = "windows")]
const RESOURCES: &str = r#"\Resources\"#;

#[cfg(target_family = "unix")]
const RESOURCES: &str = "/resources/";


fn main() {
    let project_dirs = ProjectDirs::from("io.github.maxstrid", "Person Gen", "Person Gen").unwrap();
    let data_dir = project_dirs.data_dir().to_str().unwrap();
    let resource_dir = format!("{data_dir}{RESOURCES}");

    if !Path::new(&resource_dir).exists() {
        fs::create_dir_all(&resource_dir).unwrap();
    }
    
    let project_root = env::var("PROJECT_ROOT").unwrap_or(String::from("."));

    for entry in fs::read_dir(&format!("{project_root}{RESOURCES}")).unwrap() {
        let entry = entry.unwrap().file_name();
        let entry = entry.to_str().unwrap();
        fs::copy(format!("{project_root}{RESOURCES}{entry}"), format!("{resource_dir}{entry}")).unwrap();
    }

    println!("cargo:rerun-if-changed={project_root}{RESOURCES}");
}
