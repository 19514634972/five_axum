use std::fmt::format;
use axum::response::IntoResponse;
use axum::extract::{Path, Query, Request};
use axum::http::{header, HeaderMap};
use axum::{Json, Router};
use axum::routing::{get,post};
use serde::{Deserialize, Serialize};
use std::string::String;

pub fn create_routes() ->Router{
    let get_user_id=Router::new()
        .route("/users/:user_id",get(get_user_id))
        .route("/list-products",get(list_products))
        .route("/list-header",get(list_header))
        .route("/category/:cat_id/product/:pro_id",get(get_category))
        .route("/list-user/:user_id",post(list_users));
    get_user_id
}


async fn get_user_id(Path(user_id):Path<i32>)->impl IntoResponse{

    format!("get user id from path: {user_id:}")

}

async fn get_category(Path((cat_id,pro_id)):Path<(i32,i32)>)-> String{

    format!("get user id from cat_id: {cat_id},product id:{pro_id}")

}


#[derive(Debug,Serialize,Deserialize)]
struct Pagination{
    //list-product?pageSize=25
    //#[serde(rename="pageSize")]//rust不支持-只支持_
    pageSize:Option<u32>, //default10
    page:Option<u32>,//1
}
async fn list_products(Query((pagination)):Query<(Pagination)>)->String{
    let page=pagination.page.unwrap_or(1);
    let pageSize=pagination.pageSize.unwrap_or(10);

    format!("list products from pagination:{pageSize},page={page}")

}

#[derive(Debug,Serialize,Deserialize)]
struct RequestUser{
    name:String,
    age:i32,
    email:String,
}
async fn list_header(hm:HeaderMap)->String{
  if let Some(value)=  hm.get("x-custom-header"){
      println!("x-custom-header value:{}",value.to_str().unwrap());
  }

    let mut response=String::from("");
    hm.iter().map(|(name,value)|{
        response.push((&format!("name:{:#?} value:{:#?}\n", name, value)).parse().unwrap());
        response.clone()
    }).collect()

}

//提取器3哥
async fn list_users(
    Path(user_id): Path<i32>,
    Query(pagination): Query<Pagination>,
    Json(user): Json<RequestUser>,
) -> String {
    format!("user id: {user_id}, user: {:?}", user)
}