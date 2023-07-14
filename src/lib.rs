mod models;

use async_graphql::{http::GraphiQLSource, EmptyMutation, EmptySubscription};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    response::{self, IntoResponse},
    routing, Extension, Router, Server,
};

type Schema = async_graphql::Schema<models::Query, EmptyMutation, EmptySubscription>;

async fn graphql_handler(schema: Extension<Schema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphiql() -> impl IntoResponse {
    response::Html(GraphiQLSource::build().endpoint("/").finish())
}

pub async fn run() {
    let schema = Schema::new(models::Query, EmptyMutation, EmptySubscription);

    let app = Router::new()
        .route("/", routing::get(graphiql).post(graphql_handler))
        .layer(Extension(schema));

    Server::bind(&"127.0.0.1:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
