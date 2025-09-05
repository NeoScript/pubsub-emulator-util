use std::{fs::File, io, path::Path};

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ProjectInitFile {
    pub project_id: String,
    pub topics: Vec<TopicInitEntry>,
}

#[derive(Deserialize, Debug)]
pub struct PushSubInitEntry {
    pub name: String,
    pub endpoint: String,
}

#[derive(Deserialize, Debug)]
pub struct TopicInitEntry {
    pub name: String,
    pub pull_subscriptions: Option<Vec<String>>,
    pub push_subscriptions: Option<Vec<PushSubInitEntry>>,
}

pub fn parse_init_file<P: AsRef<Path>>(path: P) -> Result<ProjectInitFile, io::Error> {
    let path = path.as_ref();
    let file = File::open(path)?;

    let init_config = serde_json::from_reader(file)?;
    Ok(init_config)
}
