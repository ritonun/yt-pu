use std::path::PathBuf;

#[derive(Debug)]
pub struct Config {
    pub output_path: PathBuf,
    pub url: String,
    pub delete_local: bool,
}

impl Config {
    pub fn new(output_path: PathBuf, url: String, delete_local: bool) -> Config {
        Config {
            output_path,
            url,
            delete_local,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Video {
    pub path: PathBuf,
    pub name: String,
    pub url: String,
}

impl Video {
    pub fn new(path: PathBuf, name: String, url: String) -> Video {
        Video { path, name, url }
    }
}
