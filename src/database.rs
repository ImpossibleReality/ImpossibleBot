use mongodb::{bson::doc, options::ClientOptions, Client, Database};
use std::env;

pub async fn connect() -> Database {
    // Parse a connection string into an options struct.
    let mut client_options = ClientOptions::parse(
        env::var("MONGO_URL").expect("Please provide MONGO_URL in the environment"),
    )
    .await
    .expect("Error parsing MONGO_URL");

    // Manually set an option.
    client_options.app_name = Some("ImpossibleBot".to_string());

    // Get a handle to the deployment.
    let client = Client::with_options(client_options).expect("Error connecting to database");

    client
        .database("impossiblebot")
        .run_command(doc! {"ping": 1}, None)
        .await
        .expect("Error connecting to MongoDB database");

    return client.database("impossiblebot");
}
