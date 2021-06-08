use crate::auth_proxy::AuthorizationDetails;
use chrono::offset::Utc;
use chrono::{DateTime, Duration};
use serde::Deserialize;
use std::ops::Add;

static MAX_QUERY_DURATION_DAYS: i64 = 30;

#[derive(Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccountActivity {
    pub action: String,
    pub symbol: String,
    pub quantity: f64,
    pub net_amount: f64,
    pub settlement_date: String,
}

#[derive(Deserialize, Debug)]
pub struct AccountActivities {
    pub activities: Vec<AccountActivity>,
}

pub async fn get_account_activities(
    authz_details: AuthorizationDetails,
    account_id: String,
    account_start_time_string: String,
) -> Result<AccountActivities, Box<dyn std::error::Error>> {
    // derive API endpoint URL
    let api_endpoint = format!(
        "{}v1/accounts/{}/activities",
        authz_details.api_server, account_id
    );
    println!("API Endpoint : {}", api_endpoint);

    // obtain activities in increments of MAX_QUERY_DURATION_DAYS days (API limit is 31 days)
    let now: DateTime<Utc> = Utc::now();
    let mut start_time: DateTime<Utc> = account_start_time_string.parse::<DateTime<Utc>>()?;
    let mut end_time: DateTime<Utc>;
    let client = reqwest::Client::new();
    // let request_builder: RequestBuilder = ;
    let mut all_activities: Vec<AccountActivity> = Vec::new();

    while now > start_time {
        // offset the end time by MAX_QUERY_DURATION_DAYS days from the start time
        end_time = start_time.add(Duration::days(MAX_QUERY_DURATION_DAYS));

        // make request and read API response
        let api_response = client
            .get(&api_endpoint)
            .bearer_auth(&authz_details.access_token)
            .query(&[
                ("startTime", start_time.to_rfc3339()),
                ("endTime", end_time.to_rfc3339()),
            ])
            .send()
            .await?
            .text()
            .await?;
        let account_activities_for_timerange: AccountActivities =
            serde_json::from_str(&api_response)?;
        println!(
            "found {} activities for range [start: {}, end: {}]",
            account_activities_for_timerange.activities.len(),
            start_time,
            end_time
        );
        all_activities.append(
            &mut account_activities_for_timerange
                .activities
                .clone()
                .into_iter()
                .filter(|x: &AccountActivity| x.action == "Buy" || x.action == "Sell")
                .collect(),
        );

        // set new start time to the end time for the next loop iteration
        start_time = end_time;
    }

    Ok(AccountActivities {
        activities: all_activities,
    })
}
