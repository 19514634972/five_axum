use axum::body::Body;
use axum::response::{IntoResponse, Response};
use axum::Router;
use axum::routing::{get,post,put,delete};
use http_body_util::BodyExt;

pub  fn create_route() ->Router{
    let user_app=Router::new()
        .route("/hi",get(say_hi))
        .route("/welcome",get(welcome).post(welcome))
        .route("/res",get(say_hi_vec_response).post(say_hi_vec_response))
        .route("/uppercase",post(echo_uppercase));
    user_app
}

async fn say_hi()->String{
    "hihiihi".to_string()
}

async fn welcome()->String{
    "welcome to string".to_string()
}

async fn say_hi_vec_response()->impl  IntoResponse{
    let resp="Hello, World!".as_bytes().to_vec();
    resp.into_response()
}



async fn echo_uppercase(req:Body)->impl  IntoResponse{
    //用戶提交的数据转换成大写
   let frame= req.map_frame(|frame|{
       frame.map_data(|data|{
           data.iter()
               .map(|byte|byte.to_ascii_uppercase())
               .collect::<axum::body::Bytes>()
       })
   });
    let resp=Response::new(frame);
    resp

}