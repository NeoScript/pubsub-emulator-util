use std::{fs::File, io, path::Path};

use serde::{Deserialize, Serialize};

use crate::pubsub::models::Topic;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProjectInitFile {
    pub project_id: String,
    pub topics: Vec<TopicInitEntry>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PushSubInitEntry {
    pub name: String,
    pub endpoint: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TopicInitEntry {
    pub name: String,
    pub pull_subscriptions: Option<Vec<String>>,
    pub push_subscriptions: Option<Vec<PushSubInitEntry>>,
}

impl From<TopicInitEntry> for Topic {
    fn from(value: TopicInitEntry) -> Self {
        Self {
            name: value.name,
            labels: None,
        }
    }
}

pub fn parse_init_file<P: AsRef<Path>>(path: P) -> Result<ProjectInitFile, io::Error> {
    let path = path.as_ref();
    let file = File::open(path)?;

    let init_config = serde_json::from_reader(file)?;
    Ok(init_config)
}
