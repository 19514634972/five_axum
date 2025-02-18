use std::net::SocketAddr;
use tracing_subscriber::fmt::time::LocalTime;
use five_axum::users::middleware::MyLayer;
use time::macros::format_description;


#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .pretty()
        .compact()
        .with_target(false)
        .with_timer(LocalTime::new(format_description!("[year]-[month]-[day] [hour]:[minute]:[second]")))
        .with_max_level(tracing::Level::DEBUG)
        .init();
    let app =axum::Router::new();
    let user_routers=five_axum::users::interactive::create_route();
    let map_json_routers=five_axum::users::map_json::create_routes();
    let get_user_id_path=five_axum::users::path_extractor::create_routes();
    let share_data=five_axum::users::extension::create_routes();
    let app=app.merge(user_routers)
        .merge(share_data)
        .merge(map_json_routers)
        .merge(get_user_id_path)
        .layer(MyLayer);

    let listener=tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener,app.into_make_service_with_connect_info::<SocketAddr>()).await.unwrap();

}


