/// Handles errors in the application.
use mongodb::bson;
use ron::ser::to_string;
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use thiserror::Error;
use unescape::unescape;
use validator::{ValidationErrors, ValidationErrorsKind};
use warp::{http::StatusCode, reply, Rejection, Reply};

/// Enum containing errors.
#[derive(Error, Debug, Clone)]
pub enum BoxOfficeError {
    //
    #[error("Validation Error: '{0}'")]
    ValidationError(#[from] ValidationErrors),
    #[error("Bad create movie request : '{0}'")]
    BadCreateMovieRequestError(String),
    #[error("Unable to process delete movie request : '{0}'")]
    BadDeleteMovieRequestError(String),
    #[error("Invalid delete movie request : '{0}'")]
    InvalidDeleteMovieRequestError(String),
    #[error("Invalid delete movie request : '{0}'")]
    InvalidUpdateMovieRequestError(String),
    #[error("Unable to process create movie request :  '{0}'")]
    InvalidFieldInCreateMovieRequestError(String),
    #[error("Unable to process update movie request : '{0}'")]
    BadUpdateMovieRequestError(String),
    #[error("Error in verdict calculation : '{0}'")]
    ErrorinVerdictCalculation(String),
    #[error("Mongodb Error: Mongo DB server is not running{0}")]
    MongoDBError(String),
    #[error("Mongodb Error: '{0}'")]
    MongoError(#[from] mongodb::error::Error),
    #[error("Error during mongodb query: '{0}'")]
    MongoQueryError(mongodb::error::Error),
    #[error("Could not access field in document: '{0}'")]
    MongoDataError(#[from] bson::document::ValueAccessError),
    #[error("Invalid Movie Id used: '{0}'")]
    InvalidMovieIDError(String),
    #[error("Document with Movie Id : '{0}' does not exist.")]
    InvalidDocumentError(String),
    #[error("Document with Movie Id : '{0}' already exist.")]
    InvalidCreateDocumentError(String),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ErrorResponse {
    pub success: bool,
    pub error_message: String,
}

#[derive(Serialize, Debug)]
struct RequestFieldError {
    request_field: String,
    request_field_errors: Vec<String>,
}

impl warp::reject::Reject for BoxOfficeError {}

/// Function to handle error.
pub async fn handle_rejection(err: Rejection) -> std::result::Result<Box<dyn Reply>, Infallible> {
    let code;
    let error_message;
    let success;

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        error_message = "Not Found".to_string();
        success = false;
        log::error!("{}", error_message);
    } else if let Some(_) = err.find::<warp::filters::body::BodyDeserializeError>() {
        code = StatusCode::BAD_REQUEST;
        error_message = "Invalid Request Body".to_string();
        success = false;
        log::error!("{}", error_message);
    } else if let Some(e) = err.find::<BoxOfficeError>() {
        match e {
            BoxOfficeError::BadCreateMovieRequestError(_) => {
                code = StatusCode::BAD_REQUEST;
                error_message = e.to_string();
                success = false;
                log::error!("{}", error_message);
            }

            BoxOfficeError::BadDeleteMovieRequestError(_) => {
                code = StatusCode::BAD_REQUEST;
                error_message = e.to_string();
                success = false;
                log::error!("{}", error_message);
            }

            BoxOfficeError::InvalidDeleteMovieRequestError(_) => {
                code = StatusCode::BAD_REQUEST;
                error_message = e.to_string();
                success = false;
                log::error!("{}", error_message);
            }

            BoxOfficeError::InvalidFieldInCreateMovieRequestError(_) => {
                code = StatusCode::PRECONDITION_FAILED;
                error_message = e.to_string();
                success = false;
                log::error!("{}", error_message);
            }

            BoxOfficeError::BadUpdateMovieRequestError(_) => {
                code = StatusCode::BAD_REQUEST;
                error_message = e.to_string();
                success = false;
                log::error!("{}", error_message);
            }

            BoxOfficeError::InvalidUpdateMovieRequestError(_) => {
                code = StatusCode::BAD_REQUEST;
                error_message = e.to_string();
                success = false;
                log::error!("{}", error_message);
            }

            BoxOfficeError::InvalidMovieIDError(_) => {
                code = StatusCode::BAD_REQUEST;
                error_message = e.to_string();
                success = false;
                log::error!("{}", error_message);
            }
            BoxOfficeError::InvalidDocumentError(_) => {
                code = StatusCode::BAD_REQUEST;
                error_message = e.to_string();
                success = false;
                log::error!("{}", error_message);
            }
            BoxOfficeError::InvalidCreateDocumentError(_) => {
                code = StatusCode::BAD_REQUEST;
                error_message = e.to_string();
                success = false;
                log::error!("{}", error_message);
            }
            BoxOfficeError::MongoDBError(_) => {
                code = StatusCode::INTERNAL_SERVER_ERROR;
                error_message = e.to_string();
                success = false;
                log::error!("{}", error_message);
            }
            BoxOfficeError::ErrorinVerdictCalculation(_) => {
                code = StatusCode::CONFLICT;
                error_message = e.to_string();
                success = false;
                log::error!("{}", error_message);
            }

            // handle validation error thrown by 'validator' crate.
            BoxOfficeError::ValidationError(validation_errors) => {
                let validation_errors_message: Vec<RequestFieldError> = validation_errors
                    .errors()
                    .iter()
                    .map(|error_kind| RequestFieldError {
                        request_field: error_kind.0.to_string(),
                        request_field_errors: match error_kind.1 {
                            ValidationErrorsKind::Struct(struct_error) => {
                                validation_error_to_string_vector(struct_error)
                            }
                            ValidationErrorsKind::Field(field_errs) => field_errs
                                .iter()
                                .map(|request_field_error| {
                                    format!(
                                        "{}: {:?}",
                                        request_field_error.code, request_field_error.params
                                    )
                                })
                                .collect(),
                            ValidationErrorsKind::List(vec_errs) => vec_errs
                                .iter()
                                .map(|validation_error| {
                                    format!(
                                        "{}: {:?}",
                                        validation_error.0,
                                        validation_error_to_string_vector(validation_error.1)
                                            .join(" | "),
                                    )
                                })
                                .collect(),
                        },
                    })
                    .collect();
                code = StatusCode::PRECONDITION_FAILED;
                // Convert validation error to string error message.
                error_message = unescape(&to_string(&validation_errors_message).unwrap()).unwrap();
                eprintln!("{}", error_message); //validation error is printed to the standard error.
                success = false;
                log::error!("{}", error_message);
            }

            _ => {
                //eprintln!("unhandled application error: {:?}", err);
                eprintln!("Internal Server Error : {:?}", err);
                code = StatusCode::INTERNAL_SERVER_ERROR;
                error_message =
                    "Internal Server Error : Mongo DB server is not running".to_string();
                success = false;
                log::error!("{}", error_message);
            }
        }
    } else if let Some(_) = err.find::<warp::reject::MethodNotAllowed>() {
        code = StatusCode::METHOD_NOT_ALLOWED;
        error_message = "Method Not Allowed".to_string();
        success = false;
        log::error!("{}", error_message);
    } else {
        //eprintln!("unhandled error: {:?}", err);
        eprintln!("Internal Server Error : {:?}", err);
        code = StatusCode::INTERNAL_SERVER_ERROR;
        error_message = "Internal Server Error".to_string();
        success = false;
        log::error!("{}", error_message);
    }
    let read_success_value = success; //This is just to remove warning 'value assigned to success is never read.'
    let json = reply::json(&ErrorResponse {
        error_message: error_message.into(),
        success: read_success_value.into(),
    });

    Ok(Box::new(reply::with_status(json, code)))
}

/// Function to convert validation error thrown by 'validator' crate into string.
fn validation_error_to_string_vector(validationerror: &ValidationErrors) -> Vec<String> {
    validationerror
        .field_errors()
        .iter()
        .map(|request_field_error| {
            format!(
                "{}: errors: {}",
                request_field_error.0,
                request_field_error
                    .1
                    .iter()
                    .map(|validationerror| format!(
                        "{}: {:?}",
                        validationerror.code, validationerror.params
                    ))
                    .collect::<Vec<String>>()
                    .join(", ")
            )
        })
        .collect()
}
