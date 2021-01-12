Chatworkの公式ドキュメント（https://developer.chatwork.com/ja/oauth.html）に則って認可トークンを取得する手続きを自動化します
# 実行方法
- 以下のように実行時にパラメーターとして渡すことができます
  - chatwork_auth -i 'アカウントID' -p 'パスワード' -c 'OAuthクライアントID' -s 'OAuthクライアントシークレット' -rs 'コンセント画面URL' -os 'トークンエンドポイントURL'
- 環境変数を設定することで、実行時のパラメーター指定を省略することができます
  - CW_OAUTH_ID: アカウントID
  - CW_OAUTH_PASS: パスワード
  - CW_OAUTH_CLIENT: OAuthクライアントID
  - CW_OAUTH_SECRET: OAuthクライアントシークレット
  - CW_RESOURCE_SERVER: コンセント画面を起動するリソースサーバーのURL
    - 未指定時は`https://www.chatwork.com/packages/oauth2/login.php`にアクセスする
  - CW_OAUTH_SERVER: 認可トークンの払い出しを行うOAuthサーバーのURL
    - 未指定時は`https://oauth.chatwork.com/token`にアクセスする
# 残
- パラメーターを使い切る（一部のパラメーターは未使用）
- スコープをパラメーターで指定できるようにする
- リダイレクトURLをパラメーターで指定できるようにする
- 取得したトークンはクリップボードに貼り付けてコマンドを終了する
- Docker化
# 免責事項
- このツールは2020/01/01時点の公式ドキュメントに則って実装されています。APIの仕様が変更された場合は正しく動かなくなります。
- 実装を簡易にするため、`unwrap`を多用しています。実行に失敗した際に不親切でわかりにくい結果となる可能性があります。