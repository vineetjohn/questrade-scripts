use reqwest;
use serde::{Deserialize, Serialize};

static REFRESH_TOKEN_IDENTIFIER: &str = "refresh_token";
static QUESTRADE_AUTH_API_ENDPOINT: &str = "https://login.questrade.com/oauth2/token";

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthorizationDetails {
    pub refresh_token: String,
    pub api_server: String,
    pub access_token: String,
}

/*
Authenticates with the Questrade service and produces the
new refresh token as well as the access token and api server
URL which will be used for subsequent API requests.
*/
pub async fn get_authorization(
    refresh_token: String,
) -> Result<AuthorizationDetails, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let response_body = client
        .get(QUESTRADE_AUTH_API_ENDPOINT)
        .query(&[
            ("grant_type", REFRESH_TOKEN_IDENTIFIER),
            (REFRESH_TOKEN_IDENTIFIER, &refresh_token),
        ])
        .send()
        .await?
        .text()
        .await?;

    let authz_details: AuthorizationDetails = serde_json::from_str(&response_body)?;

    Ok(authz_details)
}
