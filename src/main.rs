use chatwork_oauth::auth_code::auth_code::AuthCode;
use chatwork_oauth::auth_token::auth_token::AuthToken;
use chatwork_oauth::params::params::Params;
use std::process;

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

    let auth_code = match AuthCode::get_auth_code(&params) {
        Ok(code) => code,
        Err(e) => {
            println!("Error has occurred:{}", e);
            process::exit(1);
        }
    };

    let access_token = match AuthToken::get_oauth_token(&params, &auth_code) {
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
