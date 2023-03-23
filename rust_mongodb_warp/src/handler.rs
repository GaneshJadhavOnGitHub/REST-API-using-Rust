use crate::error::BoxOfficeError;
use crate::{db_layer::*, request_response_structs::*, response::*, WebResult};
/// Functions to get REST Requests and return response.
use bytes::Buf;
type Result<T> = std::result::Result<T, BoxOfficeError>;
use validator::Validate;
use warp::{http::StatusCode, reject, reply::json, Reply};

/// Handles Create Movie (POST) request.
pub async fn create_movie_handler(buf: impl Buf, db: DB) -> WebResult<impl Reply> {
    // Deserialize input and map to CreateMovieRequest.
    let des = &mut serde_json::Deserializer::from_reader(buf.reader());
    let create_movie_request: CreateMovieRequest = serde_path_to_error::deserialize(des)
        .map_err(|e| reject::custom(BoxOfficeError::BadCreateMovieRequestError(e.to_string())))?;

    // Validate request fields.
    create_movie_request
        .validate()
        .map_err(|e| reject::custom(BoxOfficeError::ValidationError(e)))?;

    // Business Validation
    let _validation_result = validate_create_movie_request(&create_movie_request, &db)
        .await
        .map_err(|e| {
            reject::custom(BoxOfficeError::InvalidFieldInCreateMovieRequestError(
                e.to_string(),
            ))
        })?;

    // Create Movie.
    let _create_movie_result = db
        .create_movie(&create_movie_request)
        .await
        .map_err(|e| reject::custom(e))?;

    // Get created Movie.
    let created_movie = db
        .get_movie_by_id(&create_movie_request.movie_id.to_string().as_str())
        .await
        .map_err(|e| reject::custom(e))?;

    // Populate Response
    let create_movie_response = populate_create_movie_response(created_movie);

    // Return Response.
    return Ok(warp::reply::with_status(
        json(&create_movie_response),
        StatusCode::CREATED,
    ));
}

/// Handles Read Movie (GET) request.
pub async fn read_movies_handler(db: DB) -> WebResult<impl Reply> {
    // Fetch Movies.
    let fetched_movies = db.read_movies().await.map_err(|e| reject::custom(e))?;
    // Populate Response
    let read_movies_response = populate_read_movies_response(fetched_movies);
    // Return Response.
    return Ok(warp::reply::with_status(
        json(&read_movies_response),
        StatusCode::OK,
    ));
}

/// Handles Update Movie (PUT) request.
pub async fn update_movie_handler(id: String, buf: impl Buf, db: DB) -> WebResult<impl Reply> {
    // Deserialize input and map to UpdateMovieRequest.
    let des = &mut serde_json::Deserializer::from_reader(buf.reader());
    let update_movie_request: UpdateMovieRequest = serde_path_to_error::deserialize(des)
        .map_err(|e| reject::custom(BoxOfficeError::BadUpdateMovieRequestError(e.to_string())))?;

    // Validate request fields.
    update_movie_request
        .validate()
        .map_err(|e| reject::custom(BoxOfficeError::ValidationError(e)))?;

    // Business Validation
    let _validation_result = validate_update_movie_id(&id, &db)
        .await
        .map_err(|e| reject::custom(BoxOfficeError::BadUpdateMovieRequestError(e.to_string())))?;

    // Update Movie
    let number_of_movies_updated = db
        .update_movie(&id, &update_movie_request)
        .await
        .map_err(|e| reject::custom(e))?;
    if number_of_movies_updated == 1 {
        let updated_movie = db
            .get_movie_by_id(&id.to_string().as_str())
            .await
            .map_err(|e| reject::custom(e))?;

        // Populate Movie Response and Returns.
        let update_movie_response = populate_update_movie_response(updated_movie);
        return Ok(warp::reply::with_status(
            json(&update_movie_response),
            StatusCode::OK,
        ));
    } else {
        let db_operation_failed_error_response = populate_db_operation_failed_error_response();
        return Ok(warp::reply::with_status(
            json(&db_operation_failed_error_response),
            StatusCode::INTERNAL_SERVER_ERROR,
        ));
    }
}

/// Handles Delete Movie (DELETE) request.
pub async fn delete_movie_handler(id: String, db: DB) -> WebResult<impl Reply> {
    // Business validation.
    let _validation_result = validate_delete_movie_id(&id, &db)
        .await
        .map_err(|e| reject::custom(BoxOfficeError::BadDeleteMovieRequestError(e.to_string())))?;

    // Delete Movie and Returns Response.
    let number_of_records_deleted = db.delete_movie(&id).await.map_err(|e| reject::custom(e))?;
    if number_of_records_deleted == 1 {
        let delete_movie_response = populate_delete_movie_response(&id.as_str());
        return Ok(warp::reply::with_status(
            json(&delete_movie_response),
            StatusCode::OK,
        ));
    } else {
        let db_operation_failed_error_response = populate_db_operation_failed_error_response();
        return Ok(warp::reply::with_status(
            json(&db_operation_failed_error_response),
            StatusCode::INTERNAL_SERVER_ERROR,
        ));
    }
}

/// Business validation for Create Movie Request (POST).
/// Checks whether Movie is already present in the Collecton or not.
/// If present then we can not create another movie with same Movie Id.
async fn validate_create_movie_request(
    create_movie_request: &CreateMovieRequest,
    db: &DB,
) -> Result<()> {
    let id_string = &create_movie_request.movie_id.to_string();
    let id = id_string.as_str();
    let movie_result = db
        .check_movie_exist(&create_movie_request.movie_id.to_string().as_str())
        .await;
    if movie_result.is_ok() {
        let movie = movie_result.unwrap();
        if *movie {
            return Err(BoxOfficeError::InvalidCreateDocumentError(id.to_string()));
        } else {
            return Ok(());
        }
    } else {
        return Err(BoxOfficeError::MongoDBError("".to_string()));
    }
} //End of validate_create_movie_request

/// Business validation for Update Movie (PUT).
/// Checks whether Movie is present in the Collecton or not.
/// If present then only we can update it.

async fn validate_update_movie_id(id: &String, db: &DB) -> Result<()> {
    let int_id = id.parse::<i32>();
    let valid_id = if let Ok(unwrapped_id) = int_id {
        unwrapped_id
    } else {
        return Err(BoxOfficeError::InvalidUpdateMovieRequestError(
            id.to_string(),
        ));
    };

    let movie_result = db.check_movie_exist(&valid_id.to_string().as_str()).await;
    if movie_result.is_ok() {
        let movie = movie_result.unwrap();
        if *movie {
            return Ok(());
        } else {
            return Err(BoxOfficeError::InvalidDocumentError(id.to_string())); //StatusCode::NOT_FOUND
        }
    } else {
        return Err(BoxOfficeError::MongoDBError("".to_string()));
    }
} //validate_update_movie_request

/// Business validation for Delete Movie (DELETE).
/// Checks whether Movie is present in the Collecton or not.
/// If present then only we can delete it.
async fn validate_delete_movie_id(id: &String, db: &DB) -> Result<()> {
    let int_id = id.parse::<i32>();
    let valid_id = if let Ok(unwrapped_id) = int_id {
        unwrapped_id
    } else {
        return Err(BoxOfficeError::InvalidDeleteMovieRequestError(
            id.to_string(),
        ));
    };

    let movie_result = db.check_movie_exist(&valid_id.to_string().as_str()).await;
    if movie_result.is_ok() {
        let movie = movie_result.unwrap();
        if *movie {
            return Ok(());
        } else {
            return Err(BoxOfficeError::InvalidDocumentError(id.to_string()));
        }
    } else {
        return Err(BoxOfficeError::MongoDBError("".to_string()));
    }
} //validate_delete_movie_request
