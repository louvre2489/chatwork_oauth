Chatworkの公式ドキュメント （https://developer.chatwork.com/ja/oauth.html） に則って認可トークンを取得する手続きを自動化します

# 実行方法
- Dockerを使ってビルドしてください
  ``` bash
  # ビルド
  docker build -t chatwork_oauth .
  ```
- `.env`ファイルを作成する
  
  ファイルには以下の内容を設定します
  ``` bash
  CW_OAUTH_ID=ログイン用メールアドレス
  CW_OAUTH_PASS=パスワード
  CW_OAUTH_CLIENT=OAuthクライアントID
  CW_OAUTH_SECRET=OAuthクライアントシークレット
  CW_OAUTH_SCOPE=スコープ（複数指定する場合はカンマ区切り）
  CW_REDIRECT_URL=リダイレクトURL（未指定時は https://example.com/callback.php）
  CW_RESOURCE_SERVER=コンセント画面を起動するリソースサーバーのURL（未指定時は https://www.chatwork.com/packages/oauth2/login.php）
  CW_OAUTH_SERVER=認可トークンの払い出しを行うOAuthサーバーのURL（未指定時は https://oauth.chatwork.com/token）
  ```
- 
  ``` bash
  # 実行
  docker run --rm --env-file .envへのパス chatwork_oauth
  
  # 以下のようにすれば.envで指定した環境変数を一部上書きして実行することができます
  docker run --rm --env-file .envへのパス -e CW_OAUTH_ID=xxxx@gmail.com chatwork_oauth 
  ```
- 以下のようにエイリアスを作っておくと便利です
  ``` bash
  以下のように設定しておくと chatwork_oauth とコマンドを実行するだけで実行できます
  alias chatwork_oauth='docker run --rm --env-file .envへのパス chatwork_oauth'
  ```

# 残
- 取得したトークンはクリップボードに貼り付けてコマンドを終了する
- リフレッシュトークンによるトークン取得
  - リフレッシュトークンの期限切れも考慮する？
    
# 免責事項
- このツールは2020/01/01時点の公式ドキュメントに則って実装されています。APIの仕様が変更された場合は正しく動かなくなります。
- 実装を簡易にするため、`unwrap`を多用しています。実行に失敗した際に不親切でわかりにくい結果となる可能性があります。
