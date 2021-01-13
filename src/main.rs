use argopt::cmd;
use chatwork_oauth::auth_code::auth_code::AuthCode;
use chatwork_oauth::auth_token::auth_token::AuthToken;
use chatwork_oauth::params::params::Params;
use std::process;

/// 参考：https://developer.chatwork.com/ja/oauth.html
#[cmd]
fn main(
    #[opt(short = "i", long = "id")] id: Option<String>,
    #[opt(short = "p", long = "password")] password: Option<String>,
    #[opt(short = "c", long = "client")] client: Option<String>,
    #[opt(short = "s", long = "secret")] secret: Option<String>,
    #[opt(short = "u", long = "scope")] scope: Option<String>,
    #[opt(short = "d", long = "redirect_url")] redirect_url: Option<String>,
    #[opt(short = "r", long = "resource_server")] resource_server: Option<String>,
    #[opt(short = "a", long = "oauth_server")] oauth_server: Option<String>,
) {
    let params = match Params::create_params(
        id,
        password,
        client,
        secret,
        scope,
        redirect_url,
        resource_server,
        oauth_server,
    ) {
        Ok(p) => p,
        Err(e) => {
            println!("Error has occurred:{}", e);
            process::exit(1);
        }
    };

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
