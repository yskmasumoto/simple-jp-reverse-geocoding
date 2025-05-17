use crate::rtree::MyPoint;
use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
};
use rstar::RTree;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// # SearchRequest
/// ## 説明
/// クエリパラメータを保持する構造体
/// - `lat`: 緯度
/// - `lon`: 経度
#[derive(Deserialize)]
pub struct SearchRequest {
    lat: f64,
    lon: f64,
}

/// # ApiResponse
/// ## 説明
/// APIのレスポンスを保持する構造体
/// - `citycode`: 市区町村コード
/// - `address`: 住所
#[derive(Serialize)]
pub struct ApiResponse {
    citycode: String,
    address: String,
}

/// # healthcheck
/// ## 引数
/// - `State(state)`: RTreeの状態を保持するArc
/// ## 戻り値
/// - `StatusCode`: HTTPステータスコード
/// - `Json<ApiResponse>`: JSON形式のレスポンス
/// ## 説明
/// ヘルスチェック用のAPIエンドポイント
/// ヘルスチェックが成功した場合、HTTPステータスコード200と"OK"のメッセージを返す
pub async fn healthcheck() -> (StatusCode, Json<ApiResponse>) {
    let response = ApiResponse {
        citycode: "".to_string(),
        address: "OK".to_string(),
    };
    (StatusCode::OK, Json(response))
}

/// # search_handler
/// ## 引数
/// - `State(state)`: RTreeの状態を保持するArc
/// - `Query(params)`: クエリパラメータを保持する構造体
/// ## 戻り値
/// - `StatusCode`: HTTPステータスコード
/// - `Json<ApiResponse>`: JSON形式のレスポンス
/// ## 説明
/// クエリパラメータで入力された緯度と経度に基づいてRTreeからポイントを検索
/// 検索結果が見つかった場合、HTTPステータスコード200とポイントの情報を返す
/// 見つからなかった場合、HTTPステータスコード404と"Not Found"のメッセージを返す
pub async fn search_handler(
    State(state): State<Arc<RTree<MyPoint>>>,
    Query(params): Query<SearchRequest>,
) -> (StatusCode, Json<ApiResponse>) {
    // クエリパラメータを取得
    let lat = params.lat;
    let lon = params.lon;

    // RTreeからポイントを検索
    let target_point = [lat, lon];
    let nearest_points = state.nearest_neighbor(&target_point);

    // is_some()でnearest_pointsが空でないことを確認
    if nearest_points.is_some() {
        tracing::info!("Nearest point found: {:?}", nearest_points);
    } else {
        tracing::warn!("No nearest point found.");
        return (
            StatusCode::NOT_FOUND,
            Json(ApiResponse {
                citycode: "".to_string(),
                address: "Not Found".to_string(),
            }),
        );
    }

    let nearest_points = nearest_points.unwrap();

    let response = ApiResponse {
        citycode: nearest_points.citycode.clone(),
        address: nearest_points.name.clone(),
    };
    (StatusCode::OK, Json(response))
}
