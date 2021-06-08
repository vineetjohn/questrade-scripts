mod activities_proxy;
mod auth_proxy;

use crate::activities_proxy::get_account_activities;
use crate::auth_proxy::{get_authorization, AuthorizationDetails};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use toml;

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

    if args.len() != 2 {
        panic!(
            "The application needs exactly 1 argument, the location of the questrade config file"
        );
    }
    let questrade_config_file_path = args[1].clone();
    println!(
        "Reading questrade config from {}",
        questrade_config_file_path
    );

    // read initial questrade config from local config file
    let initial_questrade_config = fs::read_to_string(questrade_config_file_path.clone())?;
    let mut questrade_config: QuestradeConfig = toml::from_str(&initial_questrade_config).unwrap();

    // obtain authZ details from questrade by calling the auth endpoint
    let authz_details: AuthorizationDetails =
        get_authorization(questrade_config.refresh_token).await?;
    println!("Fetched authorization details");

    // update and store the new refresh token in the local file
    questrade_config.refresh_token = authz_details.refresh_token.clone();
    let updated_questrade_config = toml::to_string(&questrade_config).unwrap();
    fs::write(questrade_config_file_path.clone(), updated_questrade_config)?;
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
    println!("all_activities: {:?}", all_activities.activities);

    // TODO: build model of the average symbol buy price

    // TODO: for each sale made, calculate ACB for a given tax year

    Ok(())
}
