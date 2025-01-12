use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use rand::{distributions::Alphanumeric, Rng};
use serde::Deserialize;
use std::{collections::HashMap, fmt::Display, sync::{Arc, RwLock}};

#[tokio::main]
async fn main() {
    let database: Database = Arc::new(RwLock::new(HashMap::<String, String>::new()));
    let app = Router::new()
        .route("/", post(create_url))
        .route("/{id}", get(get_url))
        .with_state(database);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

type Database = Arc<RwLock<HashMap<String, String>>>;

async fn create_url(database: State<Database>, Json(body): Json<Url>) -> String {
    let mut database = database.write().expect("Error obtaining the lock");
    let id: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect();
    let asdf = id.clone();

    database.insert(id, body.url);

    asdf
}

async fn get_url(database: State<Database>, Path(id): Path<String>) -> String {
    let database = database.read().expect("Error obtaining the lock");

    match database.get(&id) {
        Some(url) => url.clone(),
        None => "Not found".to_string(),
    }
}

#[derive(Deserialize)]
struct Url {
    url: String,
}

impl Display for Url {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        println!("Url: {}", self.url);
        Ok(())
    }
}
