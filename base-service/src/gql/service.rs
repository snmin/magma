use async_graphql::{EmptyMutation, EmptySubscription, Object, Schema, SimpleObject, ID};
use async_graphql_warp::graphql;
use std::convert::Infallible;
use warp::{Filter, Reply};

#[derive(SimpleObject)]
struct User {
    id: ID,
    username: String,
}

pub struct Query;

#[Object]
impl Query {
    async fn me(&self) -> User {
        User {
            id: "c".into(),
            username: "t".to_string(),
        }
    }

    #[graphql(entity)]
    async fn find_user_by_id(&self, id: ID) -> User {
        let username = if id == "1234" {
            "Me".to_string()
        } else {
            format!("User {:?}", id)
        };
        User { id, username }
    }
}