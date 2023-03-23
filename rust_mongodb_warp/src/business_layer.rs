use crate::error::*;
use rust_decimal::{prelude::*, Decimal};
/// Contains business logic.
/// Function to decide verdict depending upon calculated profit or loss.
use std::cmp::Ordering;

use crate::{constants::*, Result};

///   Decides Verdict depending upon calculated profit or loss.

/**        Following are the 10 Verdict Categories. <br/>
         1.DISASTER              (Loss > 40%) <br/>
         2.FLOP                  (Loss > 20% and Loss <= 40%) <br/>
         3.BELOW AVERAGE         (Loss <= 20% and Loss > 10%) <br/>
         4.AVERAGE               (Collection is equal to budget with 10% difference)<br/>
         5.ABOVE AVERAGE         (Profit <= 20% and Profit > 10%)   <br/>
         6.SEMI HIT              (Profit > 20% and Profit <= 40%)   <br/>
         7.HIT                   (Profit > 40% and Profit <= 80%)   <br/>
         8.SUPER HIT             (Profit > 80% and Profit <= 150%)  <br/>
         9.BLOCKBUSTER           (Profit > 150% and Profit <= 300%) <br/>
         10.ALL TIME BLOCKBUSTER (Profit > 300%)

*/
pub fn calculate_verdict(budget_crores: Decimal, collection_crores: Decimal) -> Result<String> {
    let mut verdict: &str = "";
    let loss_percentage: u16;
    let profit_percentage: u16;

    if budget_crores.cmp(&collection_crores) == Ordering::Equal {
        verdict = BUDGET_EQUALS_COLLECTION;
    } else if budget_crores.cmp(&collection_crores) == Ordering::Greater {
        let loss_percentage_result = calculate_percentage(budget_crores, collection_crores);
        if loss_percentage_result.is_ok() {
            loss_percentage = loss_percentage_result.unwrap() as u16;
            if loss_percentage > 40 {
                verdict = LOSS_PERCENTAGE_GREATER_THAN_40;
            } else if (loss_percentage > 20) && (loss_percentage <= 40) {
                verdict = LOSS_PERCENTAGE_GREATER_THAN_20_BUT_LESS_THAN_OR_EQUAL_TO_40;
            } else if (loss_percentage > 10) && (loss_percentage <= 20) {
                verdict = LOSS_PERCENTAGE_GREATER_THAN_10_BUT_LESS_THAN_OR_EQUAL_TO_20;
            } else if (loss_percentage > 0) && (loss_percentage <= 10) {
                verdict = LOSS_PERCENTAGE_GREATER_THAN_0_BUT_LESS_THAN_OR_EQUAL_TO_10;
            }
        }
    } else if budget_crores.cmp(&collection_crores) == Ordering::Less {
        let profit_percentage_result = calculate_percentage(budget_crores, collection_crores);
        if profit_percentage_result.is_ok() {
            profit_percentage = profit_percentage_result.unwrap() as u16;
            if (profit_percentage > 0) && (profit_percentage <= 10) {
                verdict = PROFIT_PERCENTAGE_GREATER_THAN_0_BUT_LESS_THAN_OR_EQUAL_TO_10;
            } else if (profit_percentage > 10) && (profit_percentage <= 20) {
                verdict = PROFIT_PERCENTAGE_GREATER_THAN_10_BUT_LESS_THAN_OR_EQUAL_TO_20;
            } else if (profit_percentage > 20) && (profit_percentage <= 40) {
                verdict = PROFIT_PERCENTAGE_GREATER_THAN_20_BUT_LESS_THAN_OR_EQUAL_TO_40;
            } else if (profit_percentage > 40) && (profit_percentage <= 80) {
                verdict = PROFIT_PERCENTAGE_GREATER_THAN_40_BUT_LESS_THAN_OR_EQUAL_TO_80;
            } else if (profit_percentage > 80) && (profit_percentage <= 150) {
                verdict = PROFIT_PERCENTAGE_GREATER_THAN_80_BUT_LESS_THAN_OR_EQUAL_TO_150;
            } else if (profit_percentage > 150) && (profit_percentage <= 300) {
                verdict = PROFIT_PERCENTAGE_GREATER_THAN_150_BUT_LESS_THAN_OR_EQUAL_TO_300;
            } else if profit_percentage > 300 {
                verdict = PROFIT_PERCENTAGE_GREATER_THAN_300;
            }
        } else {
            return Err(BoxOfficeError::ErrorinVerdictCalculation("".to_string()));
        }
    }

    return Ok(verdict.to_string());
}

///Calculates loss or profit percentage based on budget and collection of the Movie.
/// This percentage will be used to decide Verdict.

pub fn calculate_percentage(budget_crores: Decimal, collection_crores: Decimal) -> Result<u16> {
    let hundred = Decimal::new(100, 0);
    if budget_crores.cmp(&collection_crores) == Ordering::Greater
    //Loss
    {
        let loss_percentage = ((budget_crores - collection_crores) / budget_crores) * hundred;
        return Ok(loss_percentage.round().to_u16().unwrap());
    } else
    //Profit
    {
        let profit_percentage = ((collection_crores - budget_crores) / budget_crores) * hundred;
        return Ok(profit_percentage.round().to_u16().unwrap());
    }
}
