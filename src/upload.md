APP直接アップロード
```plantuml

APP -> API : アップロード用Tokenを取得
APP -> S3  : 動画をアップロード
APP -> API : アップロード結果を通知
API -> AIエンジン : 動画処理(動画URL)
AIエンジン -> S3 : 動画を取得



```

API経由アップロード
```plantuml
APP -> API : 動画をアップロード
API -> S3 : 動画をアップロード
API -> APP : アップロード結果を通知
API -> AIエンジン : 動画処理(動画URL)
AIエンジン -> S3 : 動画を取得

```