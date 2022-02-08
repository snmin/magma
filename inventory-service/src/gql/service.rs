use async_graphql::*;
use async_graphql_warp::graphql;
use sea_orm::{DatabaseConnection, EntityTrait};
use std::{convert::Infallible};
use warp::{Filter, Reply};


use crate::entitys::products::Entity as ProductEntity;

use crate::entitys::products::{Column as ProductColumns, Model as ProductModel};
use crate::models::product::Product;


#[derive(Default)]
pub struct Query;

#[Object]
impl Query {
    async fn get_product(&self, ctx: &Context<'_>, product_id: i32) -> Result<ProductModel> {
        
        let db_conn = ctx.data::<DatabaseConnection>().expect("Failed to secure db connection");
        
        let result = ProductEntity::find_by_id(product_id).one(db_conn).await?;

        Ok(result.unwrap())
    }
}
