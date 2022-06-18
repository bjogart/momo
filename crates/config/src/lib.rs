use std::path::Path;
use std::path::PathBuf;

use query_base::QueryBase;

pub trait Queries<'qs>: QueryBase {
    fn config(self) -> &'qs Config;
}

pub struct Config {
    entry: PathBuf,
}

impl Config {
    pub fn new(entry: PathBuf) -> Self {
        Self { entry }
    }

    pub fn entry(&self) -> &Path {
        &self.entry
    }
}
