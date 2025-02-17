use std::sync::Arc;
//扩展共享数据
use axum::{Extension, Router};
use axum::extract::{Request, State};
use axum::http::{Response, StatusCode};
use axum::middleware::Next;
use axum::response::IntoResponse;
use axum::routing::{get, post, put, delete};

pub struct AppSate{
    pub db:String,


}
impl AppSate{
    fn new()->Self{
        Self{
            db:"this db info".to_string()

        }
    }

    fn get_conn(&self)->String{
       self.db.clone()
    }
}


pub  fn create_routes() ->Router{
    //Arc<AppState>
  let db_data=Arc::new(AppSate::new());
    let share=Router::new()
        .route("/shared-data-1",get(share_data_1))
        .route("/shared-data-2",get(share_data_2))
        .layer(Extension(db_data))
        .route_layer()  //路由中间件使用
    share
}

async fn share_data_1(Extension(db):Extension<Arc<AppSate>>)->String{
    format!("share_data_1:{}",db.get_conn())
}

async fn share_data_2(Extension(db):Extension<Arc<AppSate>>)->String{
    format!("share_data_2:{}",db.get_conn())
}

async fn customer_middleware(req:Request,next: Next)->Result<Response<T>,StatusCode>{

    Ok("Hi".into_response())
}