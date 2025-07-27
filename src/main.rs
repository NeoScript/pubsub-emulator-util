use pubsub::topics;
use rand::{
    distr::{Alphanumeric, SampleString},
    seq::IndexedRandom,
};
use reqwest::Client;

mod pubsub;

#[tokio::main]
async fn main() {
    let client = Client::new();
    const PROJECT_ID: &str = "my-local-project";
    const HOST: &str = "http://localhost:8085";
    let topics = topics::list(PROJECT_ID, &client, HOST).await;

    match topics {
        Ok(ref result) => {
            let topics = &result.topics;
            println!("Current list of topics: {} total", topics.len());

            // topics.iter().for_each(|t| {
            //     let output =
            //         serde_json::to_string_pretty(t).expect("should be able to stringify topic");
            //     println!("{output}");
            // });
        }
        Err(ref e) => println!("Error pulling topics: {e}"),
    }

    let topic_id = Alphanumeric.sample_string(&mut rand::rng(), 8);
    let topic = topics::create(PROJECT_ID, &client, HOST, &topic_id)
        .await
        .expect("should create topic");
    println!("topic result: {topic:?}");

    let binding = topics.unwrap();
    let delete_topic = binding.topics.choose(&mut rand::rng()).unwrap();
    let delete_repsonse = topics::delete(&client, HOST, &delete_topic.name).await;

    if delete_repsonse.is_ok() {
        println!("deleted {} successfully", &delete_topic.name);
    } else {
        eprintln!("failed deleting {}", &delete_topic.name);
    }
}
