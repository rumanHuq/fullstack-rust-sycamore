use async_graphql::{
  http::{playground_source, GraphQLPlaygroundConfig},
  EmptyMutation, EmptySubscription, Object, Schema,
};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
  extract::Extension,
  handler::get,
  response::{self, IntoResponse},
  AddExtensionLayer, Router, Server,
};

#[derive(Default)]
struct Query;

#[Object]
impl Query {
  /// Returns the sum of a and b
  async fn add(&self, a: i32, b: i32) -> i32 {
    a + b
  }
}

type AppSchema = Schema<Query, EmptyMutation, EmptySubscription>;

async fn graphql_handler(schema: Extension<AppSchema>, req: GraphQLRequest) -> GraphQLResponse {
  schema.execute(req.into_inner()).await.into()
}

async fn graphql_playground() -> impl IntoResponse {
  response::Html(playground_source(GraphQLPlaygroundConfig::new("/")))
}

#[tokio::main]
async fn main() {
  let schema: AppSchema = Schema::build(Query, EmptyMutation, EmptySubscription)
    .data(Query::default())
    .finish();

  let app = Router::new()
    .route("/", get(graphql_playground).post(graphql_handler))
    .layer(AddExtensionLayer::new(schema));

  println!("Playground: http://localhost:8000");

  Server::bind(&"0.0.0.0:8000".parse().unwrap())
    .serve(app.into_make_service())
    .await
    .unwrap();
}
