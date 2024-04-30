use axum::{  
    routing::{get, patch}, Router
};

use tower_http::cors::{Any, CorsLayer};
mod handlers;

#[tokio::main]
async fn main(){

    //tracing
    tracing_subscriber::fmt::init();

    //add cors
    let cors = CorsLayer::new().allow_origin(Any);

    //add postgres
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");

    let pool = sqlx::PgPool::connect(&database_url).await.expect("Error with pool connect tion");

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        .route("/api/workspaces", get(handlers::get_workspaces).post(handlers::create_workspace))
        .route("/api/workspaces/:id", get(handlers::get_workspace))
        .with_state(pool)
        .layer(cors);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap()

}

//basic handler that responds with a static string
async fn root() -> &'static str{
    "hello, world!"
}
 

