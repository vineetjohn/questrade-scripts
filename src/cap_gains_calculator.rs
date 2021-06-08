use crate::activities_proxy::AccountActivities;
use chrono::{DateTime, Datelike};
use std::collections::HashMap;

// AccumulatedPurchases stores the point-in-time state of the
#[derive(Debug)]
struct AccumulatedPurchases {
    quantity: f64,
    amount: f64,
}

// given a chronologically ordered set of trade activities, `calculate_capital_gains`
// calculates the yearly capital gains for all securities traded.
pub async fn calculate_capital_gains(
    account_activities: AccountActivities,
) -> Result<HashMap<String, f64>, Box<dyn std::error::Error>> {
    let mut symbol_acb_tracker: HashMap<String, AccumulatedPurchases> = HashMap::new();
    let mut capital_gains_tracker: HashMap<String, f64> = HashMap::new();

    for activity in account_activities.activities.clone().into_iter() {
        // get the current state of accumulated purchases / sales
        let symbol_accumulated_buys = match symbol_acb_tracker.get(&activity.symbol) {
            Some(v) => v,
            None => {
                &(AccumulatedPurchases {
                    quantity: 0.0,
                    amount: 0.0,
                })
            }
        };

        // if the activity is a 'Sell' trade, compute ACB and derive capital gains
        if activity.action == "Sell" {
            if symbol_accumulated_buys.quantity == 0.0 {
                // handle case where the API returns bad data (duplicate trade records)
                continue;
            }

            // calculate point-in-time adjusted cost base (ACB)
            let acb_for_symbol = symbol_accumulated_buys.amount / symbol_accumulated_buys.quantity;

            // calculate capital gains
            let capital_gain_amount =
                activity.quantity * (acb_for_symbol + (activity.net_amount / activity.quantity));

            // determine tax year and add to the total gains for that year
            let tax_year = DateTime::parse_from_rfc3339(&activity.settlement_date)?
                .year()
                .to_string();
            capital_gains_tracker.insert(
                tax_year.clone(),
                match capital_gains_tracker.get(&tax_year.clone()) {
                    Some(v) => *v,
                    None => 0.0,
                } + capital_gain_amount,
            );
        }

        let new_accumulated_buys = {
            let new_quantity = symbol_accumulated_buys.quantity + activity.quantity;
            // zero out amount and quantity when the new quantity is 0, as there is no ACB to track anymore
            let new_amount = if new_quantity < 1.0 {
                0.0
            } else {
                symbol_accumulated_buys.amount + activity.net_amount
            };

            AccumulatedPurchases {
                quantity: new_quantity,
                amount: new_amount,
            }
        };
        symbol_acb_tracker.insert(activity.symbol.clone(), new_accumulated_buys);
    }

    Ok(capital_gains_tracker)
}
