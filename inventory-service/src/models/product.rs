use std::result;

use async_graphql::*;
use sea_orm::{QuerySelect, DbErr};
use sea_orm::{DatabaseConnection, EntityTrait};
use serde::{Serialize, Deserialize};
use uuid::*;


use sea_orm::{entity::*, query::*, tests_cfg::cake, DeriveColumn, EnumIter};
use warp::log;

use crate::entity::products::Entity as ProductEntity;
use crate::entity::products::Model as ProductModel;

use crate::entity::products::Column as ProductColumns;


struct Product {
    product_id: i32,
    stripe_id: String,
    product_name: String,
    product_code: String,
    
    incoming: i32,
    outgoing: i32,
    instock: i32,

    product_description: Option<String>,
    price: i32,
    magma_info: Option<Json<T>>
}

#[Object]
impl Product {

    async fn product_id( &self, ctx: &Context<'_> ) -> Result<i32> {
        
        let db_conn = ctx.data::<DatabaseConnection>().expect("Failed to secure db connection");
        
        let result = 
        ProductEntity::find_by_id(self.product_id)
            .select_only()
            .column(ProductColumns::Id)
            .one(db_conn)
            .await?;
        
        
        Ok(ProductModel::from(result.unwrap()).id)
    }

}