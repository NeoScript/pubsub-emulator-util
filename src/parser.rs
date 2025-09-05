pub struct ProjectInitFile {
    project_id: String,
    topics: Vec<TopicInitEntry>,
}

pub struct PushSubInitEntry {
    name: String,
    endpoint: String,
}

pub struct TopicInitEntry {
    topic: String,
    pull_subscriptions: Vec<String>,
    push_subscriptions: Vec<PushSubInitEntry>,
}
