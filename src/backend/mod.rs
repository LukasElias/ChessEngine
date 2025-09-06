use {
    crate::{
        Game,
        Player,
    },
    axum::{
        extract::State,
        response::{Html, IntoResponse},
        routing::get,
        Router,
    },
    std::sync::Arc,
    tokio::sync::Mutex,
};

pub async fn run_server<W, B>(game: Arc<Mutex<Game<W, B>>>)
where W: Player, B: Player {
    let app = Router::new()
        .route("/debug", get(serve_game_debug))
        .route("/", get(serve_root))
        .with_state(game);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:6464")
        .await
        .unwrap();

    axum::serve(listener, app)
        .await
        .unwrap();
}

async fn serve_game_debug<W, B>(State(game): State<Arc<Mutex<Game<W, B>>>>) -> impl IntoResponse
where W: Player, B: Player {
    let game_value = game.lock().await;

    format!("{:#?}", game_value)
}

async fn serve_root<W, B>(State(game): State<Arc<Mutex<Game<W, B>>>>) -> Html<String>
where W: Player, B: Player {
    let game_value = game.lock().await;

    Html(format!("{:#?}", game_value.current_board))
}
