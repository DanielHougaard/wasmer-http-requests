#![allow(const_item_mutation)]

use std::collections::HashMap;
wai_bindgen_rust::export!("infisical.wai");

#[allow(dead_code)]
struct Settings {
    access_token: String,
    refresh_token: String,
}

impl Settings {
    pub fn set_access_token(&mut self, access_token: String) {
        self.access_token = access_token;
    }
    pub fn set_refresh_token(&mut self, refresh_token: String) {
        self.refresh_token = refresh_token;
    }
}

const SETTINGS: Settings = Settings {
    access_token: String::new(),
    refresh_token: String::new(),
};

#[derive(serde::Deserialize)]
#[allow(non_snake_case)]
struct AccessTokenResponse {
    refreshToken: String,
    accessToken: String,
    tokenType: String,
}
struct Requests;

impl Requests {
    pub async fn get_access_token(
        refresh_token: String,
    ) -> Result<AccessTokenResponse, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();

        let mut data = HashMap::new();

        data.insert("refreshToken", refresh_token);

        let res = client
            .post("http://httpbin.org/post")
            .json(&data)
            .send()
            .await?;

        let body = res.json::<AccessTokenResponse>().await?;

        Ok(body)
    }
}

struct Infisical;

#[wai_bindgen_rust::async_trait(?Send)]
impl crate::infisical::Infisical for Infisical {
    async fn init() {
        // We have to do something like this to avoid issues with WAI
        match Requests::get_access_token("".to_string()).await {
            Ok(res) => {
                SETTINGS.set_access_token(res.accessToken);
                SETTINGS.set_refresh_token(res.refreshToken);
            }
            Err(err) => {
                // Handle the error here
                eprintln!("Error: {:?}", err);
            }
        }
    }
}
