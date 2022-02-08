use std::result;

use async_graphql::*;
use sea_orm::{QuerySelect, DbErr};
use sea_orm::{DatabaseConnection, EntityTrait};
use serde::{Serialize, Deserialize};
use uuid::*;


use sea_orm::{entity::*, query::*, tests_cfg::cake, DeriveColumn, EnumIter};
use warp::log;


use crate::entitys::products::Entity as ProductEntity;
use crate::entitys::products::{Column as ProductColumns, Model as ProductModel};


#[derive(Default)]
pub struct Product {
    product_id: i32,
    stripe_id: String,
    product_name: String,
    product_code: String,
    
    incoming: i32,
    outgoing: i32,
    instock: i32,

    product_description: String,
    price: i32,
    magma_info: Json<String>
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


    async fn stripe_id( &self, ctx: &Context<'_> ) -> Result<String> {
        
        let db_conn = ctx.data::<DatabaseConnection>().expect("Failed to secure db connection");
        
        let result = 
        ProductEntity::find_by_id(self.product_id)
            .select_only()
            .column(ProductColumns::StripeId)
            .one(db_conn)
            .await?;
        
        
        Ok(ProductModel::from(result.unwrap()).stripe_id)
    }


    async fn product_name( &self, ctx: &Context<'_> ) -> Result<String> {
        
        let db_conn = ctx.data::<DatabaseConnection>().expect("Failed to secure db connection");
        
        let result = 
        ProductEntity::find_by_id(self.product_id)
            .select_only()
            .column(ProductColumns::ProductName)
            .one(db_conn)
            .await?;
        
        
        Ok(ProductModel::from(result.unwrap()).product_name)
    }


    async fn product_code( &self, ctx: &Context<'_> ) -> Result<String> {
        
        let db_conn = ctx.data::<DatabaseConnection>().expect("Failed to secure db connection");
        
        let result = 
        ProductEntity::find_by_id(self.product_id)
            .select_only()
            .column(ProductColumns::ProductCode)
            .one(db_conn)
            .await?;
        
        
        Ok(ProductModel::from(result.unwrap()).product_code)
    }



    async fn incoming( &self, ctx: &Context<'_> ) -> Result<i32> {
        
        let db_conn = ctx.data::<DatabaseConnection>().expect("Failed to secure db connection");
        
        let result = 
        ProductEntity::find_by_id(self.product_id)
            .select_only()
            .column(ProductColumns::Incoming)
            .one(db_conn)
            .await?;
        
        
        Ok(ProductModel::from(result.unwrap()).incoming)
    }



    async fn outgoing( &self, ctx: &Context<'_> ) -> Result<i32> {
        
        let db_conn = ctx.data::<DatabaseConnection>().expect("Failed to secure db connection");
        
        let result = 
        ProductEntity::find_by_id(self.product_id)
            .select_only()
            .column(ProductColumns::Outgoing)
            .one(db_conn)
            .await?;
        
        
        Ok(ProductModel::from(result.unwrap()).outgoing)
    }



    async fn instock( &self, ctx: &Context<'_> ) -> Result<i32> {
        
        let db_conn = ctx.data::<DatabaseConnection>().expect("Failed to secure db connection");
        
        let result = 
        ProductEntity::find_by_id(self.product_id)
            .select_only()
            .column(ProductColumns::Instock)
            .one(db_conn)
            .await?;
        
        
        Ok(ProductModel::from(result.unwrap()).instock)
    }
    

    // TODO: FIGURE OUT WHY OPTIONS ARE WEIRD
    async fn product_description( &self, ctx: &Context<'_> ) -> Result<String> {
        
        // let db_conn = ctx.data::<DatabaseConnection>().expect("Failed to secure db connection");
        
        // let result = 
        // ProductEntity::find_by_id(self.product_id)
        //     .select_only()
        //     .column(ProductColumns::ProductDescription)
        //     .one(db_conn)
        //     .await?;
        
        // Ok(ProductModel::from(result.unwrap_or_else("This product has no description".to_string()).product_description))

        Ok("TEST".to_string())
    }


    async fn price( &self, ctx: &Context<'_> ) -> Result<i32> {
        
        let db_conn = ctx.data::<DatabaseConnection>().expect("Failed to secure db connection");
        
        let result = 
        ProductEntity::find_by_id(self.product_id)
            .select_only()
            .column(ProductColumns::Price)
            .one(db_conn)
            .await?;
        
        
        Ok(ProductModel::from(result.unwrap()).price)
    }

    // TODO: JSON STUFF
    async fn magma_info( &self, ctx: &Context<'_> ) -> Result<String> {
        
        // let db_conn = ctx.data::<DatabaseConnection>().expect("Failed to secure db connection");
        
        // let result = 
        // ProductEntity::find_by_id(self.product_id)
        //     .select_only()
        //     .column(ProductColumns::Price)
        //     .one(db_conn)
        //     .await?;
        
        
        // Ok(ProductModel::from(result.unwrap()).price)


        Ok("TEST".to_string())
    }


    


}