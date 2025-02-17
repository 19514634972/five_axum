use axum::{Json, Router};
use axum::routing::{get,post,put,delete};
use serde::{Deserialize, Serialize};

pub fn create_routes() ->Router{
    let json_map_routes=Router::new()
        .route("/json_demo",post(map_json_struct));
    json_map_routes
}


#[derive(Debug,Serialize,Deserialize)]
pub struct Todo{
    title:String,
    detail:String,
    is_completed:bool,
}

#[derive(Debug,Serialize,Deserialize)]
pub struct TodoResponse{
    title:String,
    message:String,
}
async fn map_json_struct(Json(req):Json<Todo>)->Json<TodoResponse>{
  let res=TodoResponse{
      title:req.title,
      message:"get data success".to_string()
  };

  Json(res)
}