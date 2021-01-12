use anyhow::Result;
use envconfig::Envconfig;

/// 環境変数を取得するための構造体
#[derive(Envconfig, Debug)]
pub struct EnvParams {
    #[envconfig(from = "CW_OAUTH_ID")]
    pub id: Option<String>,
    #[envconfig(from = "CW_OAUTH_PASS")]
    pub pass: Option<String>,
    #[envconfig(from = "CW_OAUTH_CLIENT")]
    pub client: Option<String>,
    #[envconfig(from = "CW_OAUTH_SECRET")]
    pub secret: Option<String>,
    #[envconfig(from = "CW_OAUTH_SCOPE")]
    pub scope: Option<String>,
    #[envconfig(from = "CW_REDIRECT_URL")]
    pub redirect_url: Option<String>,
    #[envconfig(from = "CW_RESOURCE_SERVER")]
    pub resource_server: Option<String>,
    #[envconfig(from = "CW_OAUTH_SERVER")]
    pub oauth_server: Option<String>,
}

impl EnvParams {
    pub fn get_env() -> Result<EnvParams> {
        Ok(Self::init_from_env()?)
    }
}
