use chrono::offset::Utc;
use chrono::DateTime;

use reqwest::RequestBuilder;
use serde::Deserialize;

use crate::auth_proxy::AuthorizationDetails;

#[derive(Deserialize, Debug)]
pub struct AccountActivity {
    pub action: String,
    pub symbol: String,
}

#[derive(Deserialize, Debug)]
pub struct AccountActivities {
    pub activities: Vec<AccountActivity>,
}

pub async fn get_account_activities(
    authz_details: AuthorizationDetails,
    account_id: String,
    start_time_string: &str,
    end_time_string: &str,
) -> Result<AccountActivities, Box<dyn std::error::Error>> {
    let api_endpoint = format!(
        "{}v1/accounts/{}/activities",
        authz_details.api_server, account_id
    );
    println!("API Endpoint : {}", api_endpoint);

    let start_time: DateTime<Utc> = start_time_string.parse::<DateTime<Utc>>()?;
    let end_time: DateTime<Utc> = end_time_string.parse::<DateTime<Utc>>()?;
    println!("start: {}, end: {}", start_time, end_time);

    let client = reqwest::Client::new();
    let request_builder: RequestBuilder = client
        .get(api_endpoint)
        .bearer_auth(authz_details.access_token)
        .query(&[
            ("startTime", &start_time_string),
            ("endTime", &end_time_string),
        ]);

    let api_response = request_builder.send().await?.text().await?;
    println!("api_response: {}", api_response);
    let account_activities: AccountActivities = serde_json::from_str(&api_response)?;

    Ok(account_activities)
}
