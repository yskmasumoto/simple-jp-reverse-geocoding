use rstar::{AABB, PointDistance, RTree, RTreeObject};
use std::path::Path;

// 地球の平均半径 (キロメートル)
const EARTH_RADIUS_KM: f64 = 6371.0;

/// # MyPoint
/// ## 説明
/// RTreeObjectを実装するための構造体
#[derive(Debug)]
pub struct MyPoint {
    lat: f64,
    lon: f64,
    pub citycode: String,
    pub name: String,
}

/// # RTreeObject
/// ## 説明
/// RTreeObjectトレイトを実装するための構造体
/// RTreeObjectトレイトは、RTreeのオブジェクトが持つべきメソッドを定義している。
impl RTreeObject for MyPoint {
    type Envelope = AABB<[f64; 2]>;

    // RTreeObjectトレイトのメソッドを実装
    fn envelope(&self) -> Self::Envelope {
        AABB::from_point([self.lat, self.lon])
    }
}

/// # PointDistance
/// ## 説明
/// PointDistanceトレイトを実装するための構造体
/// RTreeObjectを実装した構造体に対して、Haversine距離を計算するためのトレイトを実装
impl PointDistance for MyPoint {
    fn distance_2(&self, point_coords: &[f64; 2]) -> f64 {
        // 緯度経度をラジアンに変換
        let lat1_rad = self.lat.to_radians();
        let lon1_rad = self.lon.to_radians();
        let lat2_rad = point_coords[0].to_radians();
        let lon2_rad = point_coords[1].to_radians();

        // 緯度経度の差を計算
        let dlat = lat2_rad - lat1_rad;
        let dlon = lon2_rad - lon1_rad;

        // Haversine距離を計算
        let a = (dlat / 2.0).sin().powi(2)
            + lat1_rad.cos() * lat2_rad.cos() * (dlon / 2.0).sin().powi(2);
        let c = 2.0 * a.sqrt().asin();
        let distance = EARTH_RADIUS_KM * c;

        // 距離の2乗を返す
        distance * distance
    }
}

/// # make_rtree
/// ## 引数
/// - `base_path_str`: shapefileのパスを示す文字列
/// ## 戻り値
/// - `RTree<MyPoint>`: RTreeオブジェクト
/// ## 説明
/// 指定されたパスからshapefileを読み込み、RTreeを作成
/// shapefileデータから、行政区域コード、地名を取得してMyPoint構造体に格納
/// その後、RTreeを作成して返す
/// データを取得する部分は国土数値情報の住居表示住所に関するshapefileを想定
pub fn make_rtree(base_path_str: String) -> RTree<MyPoint> {
    // base_pathをPathに変換
    let base_path = Path::new(&base_path_str);
    let shp_path = base_path.with_extension("shp");

    // shapefileのReaderを作成
    let mut shp_reader = shapefile::Reader::from_path(shp_path.clone()).unwrap();

    // 点群のベクタを作成
    let mut mypoints: Vec<MyPoint> = Vec::new();

    // shpデータから緯度経度を取得してlat_lon_vecに格納、dbfデータから地名を取得してname_vecに格納
    for result in shp_reader.iter_shapes_and_records() {
        let (shape, record) = result.unwrap();

        // shapeを型に応じて処理
        // 今回使用するshpはPoint型でデータが格納されているため、Point型の処理を実装
        match shape {
            shapefile::Shape::Point(pt) => {
                // 緯度経度を取得
                let lat = pt.y;
                let lon = pt.x;

                // Recordのjusho1フィールドを取得してname_vecに格納
                // 例: Character(Some("南加賀屋四丁目"))
                let citycode = record
                    .get("city_code")
                    .and_then(|v| match v {
                        shapefile::dbase::FieldValue::Character(Some(s)) => Some(s.clone()),
                        _ => None,
                    })
                    .unwrap_or_default();

                let name = record
                    .get("jusho1")
                    .and_then(|v| match v {
                        shapefile::dbase::FieldValue::Character(Some(s)) => Some(s.clone()),
                        _ => None,
                    })
                    .unwrap_or_default();

                mypoints.push(MyPoint {
                    lat,
                    lon,
                    citycode,
                    name,
                });
            }
            _ => {
                // 他の型は無視
                tracing::info!("Unsupported shape type");
            }
        }
    }

    tracing::debug!("Number of points loaded from shapefile: {}", mypoints.len());

    // RTreeを作成
    let rtree = RTree::bulk_load(mypoints);
    // RTreeのサイズを表示
    tracing::debug!("RTree size: {}", rtree.size());

    // RTreeの最初の要素を表示して内容を確認
    if let Some(first_point) = rtree.iter().next() {
        tracing::debug!("First point in RTree: {:?}", first_point);
    } else {
        tracing::warn!("RTree is empty.");
    }

    // 完成したRTreeを返す
    rtree
}
