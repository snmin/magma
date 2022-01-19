
mod gql;

use sea_orm::DatabaseConnection;
use sea_orm::Database;
use async_graphql::{EmptyMutation, EmptySubscription, Object, Schema, SimpleObject, ID};
use async_graphql_warp::graphql;
use gql::service::Query;
use std::convert::Infallible;
use warp::{Filter, Reply};


#[tokio::main]
async fn main() {

    let db = Database::connect("postgresql://postgres:magma@host/magma").await;


    let schema = Schema::build(Query, EmptyMutation, EmptySubscription).data(db).finish();


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