use async_graphql::{EmptyMutation, EmptySubscription, Schema, http::GraphiQLSource};
use async_graphql_poem::GraphQL;
use poem::{Server, Route, listener::TcpListener, get, IntoResponse, handler, web::Html};

pub mod query;

#[handler]
async fn graphiql() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint("/").finish())
}


pub async fn start_server() {
    let schema = Schema::build(query::Query, EmptyMutation, EmptySubscription)
    .finish();

    let app = Route::new().at("/", get(graphiql).post(GraphQL::new(schema)));

    Server::new(TcpListener::bind("127.0.0.1:8000"))
        .run(app)
        .await
        .unwrap();
}