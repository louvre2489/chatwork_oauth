use crate::params::EnvParams;
use anyhow::Result;

/// 認可取得に使用するパラメーターを格納するための構造体
#[derive(Debug)]
pub struct Params {
    pub id: String,
    pub pass: String,
    pub client: String,
    pub secret: String,
    pub scope: String,
    pub redirect_url: String,
    pub resource_server: String,
    pub oauth_server: String,
}

impl Params {
    /// コマンドライン引数でID/PASS/CLIENT/SECRETが指定されている場合は引数を採用する
    /// コマンドライン引数の指定がない場合は環境変数を採用する
    /// どちらも指定がなければエラーとする
    pub fn get_params(
        id: Option<String>,
        password: Option<String>,
        client: Option<String>,
        secret: Option<String>,
        scope: Option<String>,
        redirect_url: Option<String>,
        resource_server: Option<String>,
        oauth_server: Option<String>,
    ) -> Result<Params> {
        let env = EnvParams::get_env().unwrap();
        println!("{:?}", env);

        Ok(Params {
            id: Self::get_id(id, env.id)?,
            pass: Self::get_password(password, env.pass)?,
            client: Self::get_client(client, env.client)?,
            secret: Self::get_secret(secret, env.secret)?,
            scope: Self::get_scope(scope, env.scope)?,
            redirect_url: Self::get_redirect_url(redirect_url, env.redirect_url),
            resource_server: Self::get_resource_server(resource_server, env.resource_server),
            oauth_server: Self::get_oauth_server(oauth_server, env.oauth_server),
        })
    }

    fn get_id(arg: Option<String>, env: Option<String>) -> Result<String> {
        match arg {
            Some(arg) => Ok(arg),
            None => match env {
                Some(env) => Ok(env),
                None => panic!("IDを指定してください。"),
            },
        }
    }

    fn get_password(arg: Option<String>, env: Option<String>) -> Result<String> {
        match arg {
            Some(arg) => Ok(arg),
            None => match env {
                Some(env) => Ok(env),
                None => panic!("パスワードを指定してください。"),
            },
        }
    }

    fn get_client(arg: Option<String>, env: Option<String>) -> Result<String> {
        match arg {
            Some(arg) => Ok(arg),
            None => match env {
                Some(env) => Ok(env),
                None => panic!("OAuthクライアントを指定してください。"),
            },
        }
    }

    fn get_secret(arg: Option<String>, env: Option<String>) -> Result<String> {
        match arg {
            Some(arg) => Ok(arg),
            None => match env {
                Some(env) => Ok(env),
                None => panic!("OAuthシークレットを指定してください。"),
            },
        }
    }

    fn get_scope(arg: Option<String>, env: Option<String>) -> Result<String> {
        match arg {
            Some(arg) => Ok(arg),
            None => match env {
                Some(env) => Ok(env),
                None => panic!("スコープを指定してください。"),
            },
        }
    }

    /// 指定が無い場合は、`https://example.com/callback.php`
    fn get_redirect_url(arg: Option<String>, env: Option<String>) -> String {
        match arg {
            Some(arg) => arg,
            None => match env {
                Some(env) => env,
                None => String::from("https://example.com/callback.php"),
            },
        }
    }

    /// 指定が無い場合は、`https://www.chatwork.com/packages/oauth2/login.php`
    fn get_resource_server(arg: Option<String>, env: Option<String>) -> String {
        match arg {
            Some(arg) => arg,
            None => match env {
                Some(env) => env,
                None => String::from("https://www.chatwork.com/packages/oauth2/login.php"),
            },
        }
    }

    /// 指定が無い場合は、`https://oauth.chatwork.com/token`
    fn get_oauth_server(arg: Option<String>, env: Option<String>) -> String {
        match arg {
            Some(arg) => arg,
            None => match env {
                Some(env) => env,
                None => String::from("https://oauth.chatwork.com/token"),
            },
        }
    }
}
