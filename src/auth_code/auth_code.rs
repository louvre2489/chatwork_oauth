use crate::params::params::Params;
use headless_chrome::{Browser, LaunchOptionsBuilder};

#[derive(Debug)]
pub struct AuthCode {
    pub code: String,
}

impl AuthCode {
    pub fn get_auth_code(params: &Params) -> Result<AuthCode, failure::Error> {
        let browser = Browser::new(
            LaunchOptionsBuilder::default()
                .headless(true) // falseにすると、実際にブラウザが起動するようになる
                .build()
                .unwrap(),
        )?;
        let tab = browser.wait_for_initial_tab()?;

        // ref: https://developer.chatwork.com/ja/oauth.html の 例.コンセント画面を表示するためのURL
        let url = [
            &params.resource_server,
            "?response_type=code",
            "&redirect_uri=",
            &params.redirect_url,
            "&client_id=",
            &params.client,
            "&scope=",
            &params.scope,
        ]
        .concat();

        // URLにアクセス
        tab.navigate_to(&url)?;

        // ID/PASSを入力してログイン
        tab.wait_for_element("input#login_email")?.click()?;
        tab.type_str(&params.id)?;
        tab.wait_for_element("input#login_password")?.click()?;
        tab.type_str(&params.pass)?;
        tab.wait_for_element("#login_button")?.click()?;

        tab.wait_for_element("#approve_button")?.click()?;

        let result = tab.wait_for_element("#_cw-gtm");

        let redirect_url = match result {
            Ok(_) => {
                panic!("正常終了しちゃった");
            }
            Err(_) => tab.get_url(),
        };

        // URLから認可コードを抽出する
        let w: Vec<&str> = redirect_url.rsplit('=').collect();
        let code = w[0];

        Ok(Self {
            code: String::from(code),
        })
    }
}
