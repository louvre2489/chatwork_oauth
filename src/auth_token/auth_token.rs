use crate::auth_code::auth_code::AuthCode;
use crate::params::params::Params;
use anyhow::Result;
use reqwest::header::{HeaderMap, HeaderValue};
use serde_json::Value;
use tokio;

pub struct AuthToken {}

impl AuthToken {
    pub fn get_oauth_token(params: &Params, auth_code: &AuthCode) -> Result<String> {
        // OAuthクライアントの情報をBase64エンコーディング
        let base64_value = Self::get_base64_value(&params.client, &params.secret);

        // アクセストークンを取得する
        let req = async {
            let res = Self::access_token_endpoint(&params, &auth_code, &base64_value);
            res.await
        };

        tokio::runtime::Runtime::new().unwrap().block_on(req)
    }

    /// tokenエンドポイントへのリクエスト時にAuthorizationヘッダに指定するBase64値を生成する
    fn get_base64_value(client: &str, secret: &str) -> String {
        base64::encode([client, ":", secret].join(""))
    }

    /// tokenエンドポイントにリクエストを送る
    async fn access_token_endpoint(
        params: &Params,
        auth_code: &AuthCode,
        base64_value: &str,
    ) -> Result<String> {
        let code = &auth_code.code;
        let req_body = [
            ("grant_type", "authorization_code"),
            ("code", &code),
            ("redirect_uri", &params.redirect_url),
        ];

        let client = reqwest::Client::new();
        let res: Value = client
            .post(&params.oauth_server)
            .headers(Self::construct_headers(&base64_value))
            .form(&req_body)
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
            HeaderValue::from_str(&Self::create_auth_header_value(base64_value)).unwrap(),
        );

        headers
    }

    fn create_auth_header_value(base64_value: &str) -> String {
        format!("{}{}", "Basic ", base64_value)
    }
}
