/// Map mongodb document fields to rust structure.
use rust_decimal::prelude::*;
use serde::{Deserialize, Serialize};

/// This structure is used to map mongodb document fields to rust structure.
#[derive(Serialize, Deserialize, Debug)]
pub struct Movie {
    pub id: String,
    pub movie_id: u8,
    pub title: String,
    pub production_house: String,
    pub year_of_release: u16,
    pub budget_crores: Decimal,
    pub collection_crores: Decimal,
    pub verdict: String,
}
