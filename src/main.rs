
#[tokio::main]
async fn main() {
    let app =axum::Router::new();
    let user_routers=five_axum::users::interactive::create_route();
    let map_json_routers=five_axum::users::map_json::create_routes();
    let get_user_id_path=five_axum::users::path_extractor::create_routes();
    let share_data=five_axum::users::extension::create_routes();
    let app=app.merge(user_routers)
        .merge(share_data)
        .merge(map_json_routers)
        .merge(get_user_id_path);

    let listener=tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener,app).await.unwrap();

}


