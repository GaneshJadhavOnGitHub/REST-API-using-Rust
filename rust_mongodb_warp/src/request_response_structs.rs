use crate::{constants::*, model::*};
/// Definitions for request-response structures and custom validation functions.
use rust_decimal::prelude::*;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

///Request Structure to read Create Movie Request
#[derive(Serialize, Deserialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateMovieRequest {
    #[validate(custom = "validate_movie_id")]
    pub movie_id: u8,
    #[validate(length(min = 1))]
    pub title: String,
    #[validate(length(min = 1))]
    pub production_house: String,
    #[validate(custom = "validate_year_of_release")]
    pub year_of_release: u16,
    #[validate(custom = "validate_budget_greater_than_zero")]
    pub budget_crores: Decimal,
    #[validate(custom = "validate_collection_greater_than_zero")]
    pub collection_crores: Decimal,
}

///Request Structure to read Update Movie Request
#[derive(Serialize, Deserialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateMovieRequest {
    #[validate(length(min = 1))]
    pub title: String,
    #[validate(length(min = 1))]
    pub production_house: String,
    #[validate(custom = "validate_year_of_release")]
    pub year_of_release: u16,
    #[validate(custom = "validate_budget_greater_than_zero")]
    pub budget_crores: Decimal,
    #[validate(custom = "validate_collection_greater_than_zero")]
    pub collection_crores: Decimal,
}

///Response Structure to get Create Movie Response
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateMovieResponse {
    pub success: bool,
    pub message: String,
    pub data: Movie,
}

///Response Structure to get Read Movies Response
#[derive(Serialize, Deserialize, Debug)]
pub struct ReadMoviesResponse {
    pub success: bool,
    pub message: String,
    pub data: Vec<Movie>,
}

///Response Structure to get Update Movie Response
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateMovieResponse {
    pub success: bool,
    pub message: String,
    pub data: Movie,
}

///Response Structure to get Delete Movie Response
#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteMovieResponse {
    pub success: bool,
    pub message: String,
}

/// Response structure to get failure message if database operation failed.
#[derive(Serialize, Deserialize, Debug)]
pub struct DatabaseOperationFailedErrorResponse {
    pub success: bool,
    pub message: String,
}

//*  Custom validation methods for validator type. */
/// Checks whether budget is greater than zero or not.
pub fn validate_budget_greater_than_zero(
    budget_crores: &Decimal,
) -> std::result::Result<(), ValidationError> {
    if budget_crores <= &(Decimal::ZERO) {
        return Err(ValidationError::new(BUDGET_GREATER_THAN_ZERO_ERROR));
    }
    Ok(())
}

/// Checks whether collection is greater than zero or not.
pub fn validate_collection_greater_than_zero(
    collection_crores: &Decimal,
) -> std::result::Result<(), ValidationError> {
    if collection_crores <= &(Decimal::ZERO) {
        return Err(ValidationError::new(COLLECTION_GREATER_THAN_ZERO_ERROR));
    }
    Ok(())
}

/// Checks whether year of release is between specified range.
pub fn validate_year_of_release(year_of_release: u16) -> std::result::Result<(), ValidationError> {
    if (year_of_release < MINIMUM_YEAR_OF_RELEASE) || (year_of_release > MAXIMUM_YEAR_OF_RELEASE) {
        return Err(ValidationError::new(YEAR_OF_RELEASE_BETWEEN_MIN_AND_MAX));
    }
    Ok(())
}

/// Checks whether Movie Id is greater than zero or not.
pub fn validate_movie_id(movie_id: u8) -> std::result::Result<(), ValidationError> {
    if movie_id <= ZERO {
        return Err(ValidationError::new(MOVIEID_GREATER_THAN_ZERO_ERROR));
    }
    Ok(())
}
