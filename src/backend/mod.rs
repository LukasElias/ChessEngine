use axum::{
    response::Html,
    routing::get,
    Router,
};

pub async fn run_server() {
    let app = Router::new().route("/", get(serve));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:6464").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn serve() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
