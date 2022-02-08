use sea_orm::DatabaseConnection;
use sea_orm::Database;
use sea_orm::ConnectOptions;
use async_graphql::{EmptyMutation, EmptySubscription, Object, Schema, SimpleObject, ID};
use async_graphql_warp::graphql;
use gql::service::Query;
use std::convert::Infallible;
use std::time::Duration;
use warp::{Filter, Reply};

mod gql;
mod models;
mod entitys;

#[tokio::main]
async fn main() {

    let mut opt = ConnectOptions::new("postgresql://hypermind:magma.service.postgres@localhost/magma".to_owned());
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .sqlx_logging(true);
    
    let db = Database::connect(opt).await;
    
    let schema = Schema::build(Query, EmptyMutation, EmptySubscription)
    .data(db)
    .finish();


    warp::serve(graphql(schema).and_then(
        |(schema, request): (
            Schema<Query, EmptyMutation, EmptySubscription>,
            async_graphql::Request,
        )| async move {
            Ok::<_, Infallible>(warp::reply::json(&schema.execute(request).await).into_response())
        },
    ))
    .run(([0, 0, 0, 0], 7979))
    .await;

    
}