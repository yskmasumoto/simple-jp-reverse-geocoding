# simple-jp-reverse-geocoding
国土数値情報のshpファイルを使用して緯度経度から地名を取得するAPI

## 必要要件
- [国土数値情報の住居表示住所](https://www.gsi.go.jp/kihonjohochousa/jukyo_jusho.html)のデータ
- CargoによるRustの実行環境
- 環境変数 `SHAPEFILE_PATH` に上記データのパス指定

## インストール
```bash
git clone https://github.com/yskmasumo/simple-jp-reverse-geocoding.git
cd simple-jp-reverse-geocoding
cargo build --release
```

## 環境変数
- SHAPEFILE_PATH
  国土数値情報から取得したshapefile (.shp) のパス。拡張子は不要です。

## 実行方法
```bash
export SHAPEFILE_PATH=/path/to/your/shapefile
cargo run --release
```
サーバーは `0.0.0.0:3000` で起動します。

## APIエンドポイント

### GET /healthcheck
ヘルスチェック用。常に200と`OK`を返します。
```bash
curl http://localhost:3000/healthcheck
```
```json
{"citycode":"","address":"OK"}
```

### GET /search?lat=<緯度>&lon=<経度>
緯度経度から最寄りの住所を返却します。

- **パラメータ**
  - lat: 緯度 (例: 35.6895)
  - lon: 経度 (例: 139.6917)

```bash
# googlemapで調査した住之江区役所の緯度経度を入力
curl "http://localhost:3000/search?lat=34.60971723615912&lon=135.4826683503556"
```

- **成功 (200)**
```json
{"citycode":"27125","address":"御崎三丁目"}
```

- **失敗 (404)**
```json
{"citycode":"","address":"Not Found"}
```
