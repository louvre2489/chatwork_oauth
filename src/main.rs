use anyhow::Result;
use chatwork_oauth::auth_code::auth_code::AuthCode;
use chatwork_oauth::params::params::Params;
use reqwest::header::{HeaderMap, HeaderValue};
use serde_json::Value;
use std::process;
use tokio;

/// 参考：https://developer.chatwork.com/ja/oauth.html
fn main() {
    let params = match Params::get_params() {
        Ok(p) => p,
        Err(e) => {
            println!("Error has occurred:{}", e);
            process::exit(1);
        }
    };

    println!("パラメーター:{:?}", params);

    // OAuthクライアントの情報をBase64エンコーディング
    let base64_value = get_base64_value(&params.client, &params.secret);

    let auth_code = match AuthCode::get_auth_code(&params) {
        Ok(code) => code,
        Err(e) => {
            println!("Error has occurred:{}", e);
            process::exit(1);
        }
    };

    let access_token = match get_oauth_token(auth_code, &base64_value) {
        Ok(r) => r,
        Err(e) => {
            println!("Error has occurred:{}", e);
            process::exit(1);
        }
    };

    println!("{}", "アクセストークンの取得に成功しました");
    println!("{}", "------------------------------------");
    println!("{}", access_token);

    // TODO クリップボードに貼り付ける

    // TODO リフレッシュトークンの取得
}

fn get_oauth_token(auth_code: AuthCode, base64_value: &str) -> Result<String> {
    // アクセストークンを取得する
    let req = async {
        let res = access_token_endpoint(auth_code, &base64_value);
        res.await
    };

    let access_token = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(req)
        .unwrap();

    Ok(access_token)
}

/// tokenエンドポイントへのリクエスト時にAuthorizationヘッダに指定するBase64値を生成する
fn get_base64_value(client: &str, secret: &str) -> String {
    base64::encode([client, ":", secret].join(""))
}

/// tokenエンドポイントにリクエストを送る
async fn access_token_endpoint(auth_code: AuthCode, base64_value: &str) -> Result<String> {
    let code = auth_code.code;
    let params = [
        ("grant_type", "authorization_code"),
        ("code", &code),
        ("redirect_uri", "https://example.com/callback.php"),
    ];

    let client = reqwest::Client::new();
    let res: Value = client
        .post("https://oauth.chatwork.com/token")
        .headers(construct_headers(&base64_value))
        .form(&params)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    let access_token = &res["access_token"].as_str().unwrap();

    Ok(access_token.to_string())
}

fn construct_headers(base64_value: &str) -> HeaderMap {
    let mut headers = HeaderMap::new();

    headers.insert(
        reqwest::header::CONTENT_TYPE,
        HeaderValue::from_static("application/x-www-form-urlencoded"),
    );

    headers.insert(
        reqwest::header::AUTHORIZATION,
        HeaderValue::from_str(&create_auth_header_value(base64_value)).unwrap(),
    );

    headers
}

fn create_auth_header_value(base64_value: &str) -> String {
    format!("{}{}", "Basic ", base64_value)
}
