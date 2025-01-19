use warp::Filter;
use std::env;

#[tokio::main]
async fn main() {
    println!("STARTED");
    let current_dir = env::current_dir().expect("can't get current directory");

    let api_route = warp::path!("api" / String)
        .and(warp::query::<std::collections::HashMap<String, String>>())
        .map(|function_name: String, params: std::collections::HashMap<String, String>| {
            let response = format!(
                "Func '{}', params: {:?}",
                function_name, params
            );
            warp::reply::html(response)
        });

    let file_server = warp::fs::dir(current_dir);
    let routes = api_route.or(file_server);
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
