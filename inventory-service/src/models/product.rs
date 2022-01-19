use async_graphql::*;
use serde::{Serialize, Deserialize};

mod entity;


// #[derive(Serialize, Deserialize, Debug)]
// struct Product {
//     id: i32,
//     name: String,
//     code: String,
//     description: String,
//     in_stock: u32,
//     unit_blip: String,    
// }


#[Object]
impl Product {
    async fn name( &self, ctx: &Context<'_>, id: i32) -> Result<String> {
        let this_thing: Option<product::Model> = 
    }
}