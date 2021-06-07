mod activities_proxy;
mod auth_proxy;

use crate::activities_proxy::get_account_activities;
use crate::auth_proxy::{get_authorization, AuthorizationDetails};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use toml;

static QUESTRADE_CONFIG_FILE_PATH: &str = "Questrade.toml";

#[derive(Serialize, Deserialize, Debug)]
struct QuestradeConfig {
    account_id: String,
    account_start_time: String,
    refresh_token: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    println!("Application args: {:?}", args);

    // read initial questrade config from local config file
    let initial_questrade_config = fs::read_to_string(QUESTRADE_CONFIG_FILE_PATH)?;
    let mut questrade_config: QuestradeConfig = toml::from_str(&initial_questrade_config).unwrap();

    // obtain authZ details from questrade by calling the auth endpoint
    let authz_details: AuthorizationDetails =
        get_authorization(questrade_config.refresh_token).await?;
    println!("Fetched authorization details");

    // update and store the new refresh token in the local file
    questrade_config.refresh_token = authz_details.refresh_token.clone();
    let updated_questrade_config = toml::to_string(&questrade_config).unwrap();
    fs::write(QUESTRADE_CONFIG_FILE_PATH, updated_questrade_config)?;
    println!("Updated refresh token");

    // read account activities since account start
    let all_activities = get_account_activities(
        authz_details,
        questrade_config.account_id.clone(),
        questrade_config.account_start_time.clone(),
    )
    .await?;
    println!(
        "Read {} activities in total for account {}",
        all_activities.activities.len(),
        questrade_config.account_id.clone(),
    );

    Ok(())
}
