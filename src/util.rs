use serde::{Deserialize, Serialize};
use std::{error::Error, fs, path::Path};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub title: String,
    pub allowed: Vec<String>,
}

impl ::std::default::Default for Config {
    fn default() -> Self {
        Self {
            title: String::from(""),
            allowed: vec![],
        }
    }
}

impl Config {
    pub fn check_dir(&self, path: &str) -> bool {
        let path_str = String::from(path);
        self.allowed.contains(&path_str)
    }
}

pub fn get_dirs(dir: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let ns_repos = fs::read_dir(Path::new(dir))?
        .filter_map(|f| {
            f.ok().and_then(|d| {
                if d.path().is_dir() {
                    d.path()
                        .file_name()
                        .and_then(|n| n.to_str().map(|s| String::from(s)))
                } else {
                    None
                }
            })
        })
        .collect::<Vec<String>>();

    Ok(ns_repos)
}

pub fn get_files(dir: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let ns_repos = fs::read_dir(Path::new(dir))?
        .filter_map(|f| {
            f.ok().and_then(|d| {
                if d.path().is_file() {
                    d.path()
                        .file_name()
                        .and_then(|n| n.to_str().map(|s| String::from(s)))
                } else {
                    None
                }
            })
        })
        .collect::<Vec<String>>();

    Ok(ns_repos)
}
