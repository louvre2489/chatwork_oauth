use crate::params::EnvParams;
use anyhow::Result;
use clap::{App, Arg};

const ID: &str = "id";
const PASSWORD: &str = "password";
const CLIENT: &str = "client";
const SECRET: &str = "secret";
const SCOPE: &str = "scope";
const RESOURCE_SERVER: &str = "resource_server";
const OAUTH_SERVER: &str = "oauth_server";

/// 認可取得に使用するパラメーターを格納するための構造体
#[derive(Debug)]
pub struct Params {
    pub id: String,
    pub pass: String,
    pub client: String,
    pub secret: String,
    pub scope: String,
    pub resource_server: String,
    pub oauth_server: String,
}

impl Params {
    /// コマンドライン引数でID/PASS/CLIENT/SECRETが指定されている場合は引数を採用する
    /// コマンドライン引数の指定がない場合は環境変数を採用する
    /// どちらも指定がなければエラーとする
    pub fn get_params() -> Result<Params> {
        let env = EnvParams::get_env().unwrap();
        println!("{:?}", env);

        let matches = Self::get_param_def().get_matches();

        let id = matches.value_of(ID);
        let password = matches.value_of(PASSWORD);
        let client = matches.value_of(CLIENT);
        let secret = matches.value_of(SECRET);
        let scope = matches.value_of(SCOPE);
        let resource_server = matches.value_of(RESOURCE_SERVER);
        let oauth_server = matches.value_of(OAUTH_SERVER);

        Ok(Params {
            id: Self::get_id(id, env.id)?,
            pass: Self::get_password(password, env.pass)?,
            client: Self::get_client(client, env.client)?,
            secret: Self::get_secret(secret, env.secret)?,
            scope: Self::get_scope(scope, env.scope)?,
            resource_server: Self::get_resource_server(resource_server, env.resource_server),
            oauth_server: Self::get_oauth_server(oauth_server, env.oauth_server),
        })
    }

    fn get_id(arg: Option<&str>, env: Option<String>) -> Result<String> {
        match arg {
            Some(arg) => Ok(arg.to_string()),
            None => match env {
                Some(env) => Ok(env),
                None => panic!("IDを指定してください。"),
            },
        }
    }

    fn get_password(arg: Option<&str>, env: Option<String>) -> Result<String> {
        match arg {
            Some(arg) => Ok(arg.to_string()),
            None => match env {
                Some(env) => Ok(env),
                None => panic!("パスワードを指定してください。"),
            },
        }
    }

    fn get_client(arg: Option<&str>, env: Option<String>) -> Result<String> {
        match arg {
            Some(arg) => Ok(arg.to_string()),
            None => match env {
                Some(env) => Ok(env),
                None => panic!("OAuthクライアントを指定してください。"),
            },
        }
    }

    fn get_secret(arg: Option<&str>, env: Option<String>) -> Result<String> {
        match arg {
            Some(arg) => Ok(arg.to_string()),
            None => match env {
                Some(env) => Ok(env),
                None => panic!("OAuthシークレットを指定してください。"),
            },
        }
    }

    fn get_scope(arg: Option<&str>, env: Option<String>) -> Result<String> {
        match arg {
            Some(arg) => Ok(arg.to_string()),
            None => match env {
                Some(env) => Ok(env),
                None => panic!("スコープを指定してください。"),
            },
        }
    }

    /// 指定が無い場合は、`https://www.chatwork.com/packages/oauth2/login.php`
    fn get_resource_server(arg: Option<&str>, env: Option<String>) -> String {
        match arg {
            Some(arg) => arg.to_string(),
            None => match env {
                Some(env) => env,
                None => "https://www.chatwork.com/packages/oauth2/login.php".to_string(),
            },
        }
    }

    /// 指定が無い場合は、`https://oauth.chatwork.com/token`
    fn get_oauth_server(arg: Option<&str>, env: Option<String>) -> String {
        match arg {
            Some(arg) => arg.to_string(),
            None => match env {
                Some(env) => env,
                None => "https://oauth.chatwork.com/token".to_string(),
            },
        }
    }

    /// コマンドライン引数の定義
    fn get_param_def() -> App<'static, 'static> {
        App::new("chatwork_oauth")
            .version("0.1.0")
            .author("louvre2489 <louvre2489@gmail.com>")
            .about("get token of chatwork oauth")
            .arg(
                Arg::with_name(ID)
                    .help("login user id")
                    .short("i")
                    .long("id")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name(PASSWORD)
                    .help("login password")
                    .short("p")
                    .long("password")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name(CLIENT)
                    .help("oauth cliend id")
                    .short("c")
                    .long("client")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name(SECRET)
                    .help("oauth client secret")
                    .short("s")
                    .long("secret")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name(SCOPE)
                    .help("oauth scope")
                    .short("u")
                    .long("scope")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name(RESOURCE_SERVER)
                    .help("resource server url")
                    .short("r")
                    .long("resource_server")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name(OAUTH_SERVER)
                    .help("oauth server url")
                    .short("a")
                    .long("oauth_server")
                    .takes_value(true),
            )
    }
}
