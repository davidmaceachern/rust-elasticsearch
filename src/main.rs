use dotenv;
use kv_log_macro as log;

use elasticsearch::Elasticsearch;
use tide::Request;
use tide::Server;

mod cluster;

// #[derive(Debug)]
pub struct State {
    es_client: Elasticsearch,
}
#[async_std::main]
async fn main() {
    dotenv::dotenv().ok();
    femme::with_level(femme::LevelFilter::Trace);
    log::info!("Listening on port 8000");

    let es_client: Elasticsearch = make_client();
    let app = server(es_client).await;
    app.listen("127.0.0.1:8000").await.unwrap();
}

pub fn make_client() -> Elasticsearch {
    let es_client = Elasticsearch::default();
    es_client
}

async fn server(es_client: Elasticsearch) -> Server<State> {
    let mut server: Server<State> = Server::with_state(State { es_client });

    server.at("/")
        .get(|req: Request<State>| async move { Ok("Hello world!") });
    server.at("/health")
        .get(cluster::indices::health);

    server  
}
