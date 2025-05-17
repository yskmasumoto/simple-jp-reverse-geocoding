use axum::{Router, routing::get};
use std::sync::Arc;
mod handler;
mod rtree;
use std::env;

/// # main
/// ## 説明
/// サーバーのエントリーポイント
/// サーバーを起動し、ルーティングを設定
#[tokio::main]
async fn main() {
    // tracing_subscriberを初期化
    tracing_subscriber::fmt()
        .json()
        .with_max_level(tracing::Level::DEBUG)
        // ログに重複した情報を削除するために必要
        .with_current_span(false)
        // 関数名をログから削除する
        .with_target(false)
        .init();

    tracing::info!("Starting server...");

    // 環境変数を取得してbase_pathに格納
    let base_path_str = env::var("SHAPEFILE_PATH").unwrap_or_else(|_| {
        // 環境変数が設定されていない場合のデフォルトパス
        String::from("")
    });

    // base_pathが空なら設定するように促して終了
    if base_path_str.is_empty() {
        tracing::error!("SHAPEFILE_PATH is not set. Please set it to the path of the shapefile.");
        return;
    }

    // RTreeを作成
    let rtree = rtree::make_rtree(base_path_str);

    // RTreeをArcでラップしてスレッド間で共有
    let state = Arc::new(rtree);

    // ルーターに状態を追加

    // ルーターを作成
    let app = Router::new()
        // `GET /healthcheck` goes to `healthcheck`
        .route("/healthcheck", get(handler::healthcheck))
        // `GET /search` goes to `search_handler`
        .route("/search", get(handler::search_handler).with_state(state));

    // サーバーを起動
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
