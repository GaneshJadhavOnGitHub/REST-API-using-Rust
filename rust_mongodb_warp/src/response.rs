use crate::constants::*;
use crate::model::*;
/// Functions to get response.
use crate::request_response_structs::*;

/// Function to get Create Movie response.
pub fn populate_create_movie_response(created_movie: Movie) -> CreateMovieResponse {
    let movie_id = created_movie.movie_id.to_string();
    let string_message1 = "Movie with 'Movie ID : ";
    let string_message2 = movie_id.as_str();
    let string_message3 = "' created.";
    let concatenated_response_string =
        string_message1.to_owned() + string_message2 + string_message3;

    let create_movie_response = CreateMovieResponse {
        success: true,
        message: concatenated_response_string.to_string(),
        data: created_movie,
    };

    return create_movie_response;
}

/// Function to get Read all Movies response.
pub fn populate_read_movies_response(fetched_movies: Vec<Movie>) -> ReadMoviesResponse {
    let number_of_movies_fetched = fetched_movies.len().to_string();
    let string_message1 = " '";
    let string_message2 = number_of_movies_fetched.as_str();
    let string_message3 = "' movies fetched.";
    let concatenated_response_string =
        string_message1.to_owned() + string_message2 + string_message3;
    let read_movies_response = ReadMoviesResponse {
        success: true,
        message: concatenated_response_string.to_string(),
        data: fetched_movies,
    };

    return read_movies_response;
}

/// Function to get Update Movie response.
pub fn populate_update_movie_response(updated_movie: Movie) -> UpdateMovieResponse {
    //updated_movie.movie_id.to_string().as_str();
    //Error : temporary value dropped while borrowed consider using let binding.
    // So to_string() is applied first on a variable (movie_id) and later as_str() is applied on that variable.
    // instead of doing it in a single statement as updated_movie.movie_id.to_string().as_str();
    let movie_id = updated_movie.movie_id.to_string();
    let string_message1 = "Movie with 'Movie ID : ";
    let string_message2 = movie_id.as_str();
    let string_message3 = "' updated.";
    let concatenated_response_string =
        string_message1.to_owned() + string_message2 + string_message3;

    let update_movie_response = UpdateMovieResponse {
        success: true,
        message: concatenated_response_string.to_string(),
        data: updated_movie,
    };

    return update_movie_response;
}

/// Function to get Delete Movie response.
pub fn populate_delete_movie_response(movie_id: &str) -> DeleteMovieResponse {
    let string_message1 = "Movie with 'Movie ID : ";
    let string_message2 = movie_id;
    let string_message3 = "' deleted.";
    let concatenated_response_string =
        string_message1.to_owned() + string_message2 + string_message3;

    let delete_movie_response = DeleteMovieResponse {
        success: true,
        message: concatenated_response_string.to_string(),
    };

    return delete_movie_response;
}

/// Function to get response if database operation failed.
pub fn populate_db_operation_failed_error_response() -> DatabaseOperationFailedErrorResponse {
    let error_response_string = DB_OPERATION_FAILED;

    let db_operation_failed_error_response = DatabaseOperationFailedErrorResponse {
        success: false,
        message: error_response_string.to_string(),
    };

    return db_operation_failed_error_response;
}
