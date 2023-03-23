/// Performs database operations.
use crate::error::*;
use crate::{
    business_layer::*, constants::*, error::BoxOfficeError::*, model::Movie,
    request_response_structs::*, Result,
};
use futures::StreamExt;
use mongodb::bson::{doc, document::Document};
use mongodb::{options::ClientOptions, Client, Collection};
use rust_decimal::prelude::*;

/// Structure with MONGODB Client.
/// MONGODB Client is used to connect to MONGODB.
#[derive(Clone, Debug)]
pub struct DB {
    pub client: Client,
}

impl DB {
    /// Returns MONGODB Client for Connection String and Database.
    pub async fn init() -> Result<Self> {
        let mut client_options = ClientOptions::parse(CONNECTION_STRING).await?;
        client_options.app_name = Some(DB_NAME.to_string());
        let client = Client::with_options(client_options)?;
        Ok(Self { client })
    }

    /// Pings to database and checks whether database is running or not.
    /// Returns true if database is up and running else returns false.
    pub async fn check_mongodb_is_running() -> Result<bool> {
        let mut client_options = ClientOptions::parse(CONNECTION_STRING).await?;
        client_options.app_name = Some(DB_NAME.to_string());

        let client = Client::with_options(client_options)?;

        let ping_result = client
            .database(DB_NAME)
            .run_command(doc! {PINGCOMMAND: 1}, None)
            .await;
        let is_connected: bool;
        if ping_result.is_ok() {
            is_connected = true;
            log::info!("------------------------------");
            log::info!("Connected to database successfully.");
            println!("Connected to database successfully.");
        } else {
            is_connected = false;
            log::info!("------------------------------");
            log::error!("Unable to connect to database.");
            println!("Unable to connect to database.");
        }
        return Ok(is_connected);
    }

    /// Returns MONGODB Collection on which we can perform CRUD operations.
    pub fn get_collection(&self) -> Collection<Document> {
        let database = self.client.database(DB_NAME);
        let collection = database.collection::<Document>(COLLECTION);
        return collection;
    }

    /// Creates Movie using Create Movie Request.
    pub async fn create_movie(&self, create_movie_request: &CreateMovieRequest) -> Result<()> {
        let verdict: &str;
        let mut calculated_verdict = "".to_string();
        let calculate_verdict_result = calculate_verdict(
            create_movie_request.budget_crores,
            create_movie_request.collection_crores,
        );
        if calculate_verdict_result.is_ok() {
            calculated_verdict = calculate_verdict_result.unwrap();
        }
        verdict = calculated_verdict.as_str();
        let doc = doc! {
            MOVIEID: create_movie_request.movie_id.clone() as i32,
            TITLE: create_movie_request.title.clone(),
            PRODUCTION_HOUSE: create_movie_request.production_house.clone(),
            YEAR_OF_RELEASE: create_movie_request.year_of_release.clone() as i32,
            BUDGET_CRORES: create_movie_request.budget_crores.clone().to_string() as String,
            COLLECTION_CRORES: create_movie_request.collection_crores.clone().to_string() as String,
            VERDICT: verdict,
        };

        let _insert_one_result = self
            .get_collection()
            .insert_one(doc, None)
            .await
            .map_err(MongoQueryError)?;
        return Ok(());
    }

    /// Function to Read Movies.
    /// Returns a vector containing Movies.
    pub async fn read_movies(&self) -> Result<Vec<Movie>> {
        let mut cursor = self
            .get_collection()
            .find(None, None)
            .await
            .map_err(MongoQueryError)?;

        let mut movies: Vec<Movie> = Vec::new();
        while let Some(doc) = cursor.next().await {
            movies.push(self.doc_to_movie(&doc?)?);
        }
        return Ok(movies);
    }

    /// Updates Movie using Update Movie Request and Movie Id.
    /// Returns number of documents updated (1).
    pub async fn update_movie(
        &self,
        id: &str,
        update_movie_request: &UpdateMovieRequest,
    ) -> Result<u64> {
        let verdict: &str;
        let mut calculated_verdict = "".to_string();
        let calculate_verdict_result = calculate_verdict(
            update_movie_request.budget_crores,
            update_movie_request.collection_crores,
        );
        if calculate_verdict_result.is_ok() {
            calculated_verdict = calculate_verdict_result.unwrap();
        }
        verdict = calculated_verdict.as_str();

        let query = doc! {
            "MovieID": id.parse::<i32>().unwrap() as i32,
        };

        let doc_id = id.parse::<i32>().unwrap() as i32;
        let doc_title = update_movie_request.title.clone();
        let doc_production_house = update_movie_request.production_house.clone();
        let doc_year_of_release = update_movie_request.year_of_release.clone() as i32;
        let doc_budget_crores = update_movie_request.budget_crores.clone().to_string() as String;
        let doc_collection_crores =
            update_movie_request.collection_crores.clone().to_string() as String;
        let doc_verdict = verdict.to_string();

        let doc = doc! {
           "$set": {
             MOVIEID: doc_id ,
             TITLE: doc_title ,
             PRODUCTION_HOUSE: doc_production_house ,
             YEAR_OF_RELEASE: doc_year_of_release ,
             BUDGET_CRORES: doc_budget_crores ,
             COLLECTION_CRORES: doc_collection_crores ,
             VERDICT: doc_verdict
            },
        };

        let find_result = self
            .get_collection()
            .find_one(query.clone(), None)
            .await
            .map_err(MongoQueryError)?;
        let result = find_result.unwrap();

        let query1 = doc! {
            "_id": &result.get("_id"),
        };
        let bson_movie = self
            .get_collection()
            .update_many(query1, doc, None)
            .await
            .map_err(MongoQueryError)?;
        let number_of_records_modified = bson_movie.modified_count;
        return Ok(number_of_records_modified);
    }

    /// Deletes Movie using Movie Id
    /// Returns number of documents deleted (1).
    pub async fn delete_movie(&self, id: &str) -> Result<u64> {
        if id.parse::<i32>().is_ok() {
            let filter = doc! {
                    "MovieID": id.parse::<i32>().unwrap() as i32,
            };
            let delete_result = self
                .get_collection()
                .delete_one(filter, None)
                .await
                .map_err(MongoQueryError)?;
            let number_of_records_deleted = delete_result.deleted_count;
            return Ok(number_of_records_deleted);
        } else {
            return Err(BoxOfficeError::InvalidMovieIDError("".to_string()));
        }
    }

    /// Function to fetch Movie using Movie Id.
    pub async fn get_movie_by_id(&self, id: &str) -> Result<Movie> {
        let filter = doc! {
            "MovieID": id.parse::<i32>().unwrap() as i32,
        };
        let movie_option = self
            .get_collection()
            .find_one(filter, None)
            .await
            .map_err(MongoQueryError)?;
        if movie_option.is_some() {
            let movie_document = movie_option.unwrap();
            if !movie_document.is_empty() {
                //if movie present.
                let movie = self.doc_to_movie(&movie_document)?;
                return Ok(movie);
            } else {
                return Err(BoxOfficeError::InvalidMovieIDError(id.to_string()));
            }
        } else {
            return Err(BoxOfficeError::InvalidMovieIDError(id.to_string()));
        }
    }

    /// Function to check whether Movie with input Movie Id exists in the Collection or Not.
    /// Returns true if Movie is present else returns false.
    pub async fn check_movie_exist(&self, id: &str) -> Result<&bool> {
        let is_exist: &bool;
        let filter = doc! {
            "MovieID": id.parse::<i32>().unwrap() as i32,
        };
        let movie_option = self
            .get_collection()
            .find_one(filter, None)
            .await
            .map_err(MongoQueryError)?;
        if movie_option.is_some() {
            let movie_document = movie_option.unwrap();
            if !movie_document.is_empty() {
                //if movie present.
                is_exist = &true;
                return Ok(is_exist);
            } else {
                is_exist = &false;
                return Ok(is_exist);
            }
        } else {
            is_exist = &false;
            return Ok(is_exist);
        }
    }

    /// Converts BSON Document to Movie Structure.
    /// This Movie Structure is used to return response.
    fn doc_to_movie(&self, doc: &Document) -> Result<Movie> {
        let id = doc.get_object_id(ID)?;
        let movie_id = doc.get_i32(MOVIEID)?;
        let title = doc.get_str(TITLE)?;
        let production_house = doc.get_str(PRODUCTION_HOUSE)?;
        let year_of_release = doc.get_i32(YEAR_OF_RELEASE)?;
        let budget_crores = doc.get_str(BUDGET_CRORES)?;
        let collection_crores = doc.get_str(COLLECTION_CRORES)?;
        let verdict = doc.get_str(VERDICT)?;

        let movie = Movie {
            id: id.to_hex(),
            movie_id: movie_id as u8,
            title: title.to_owned(),
            production_house: production_house.to_owned(),
            year_of_release: year_of_release as u16,
            budget_crores: Decimal::from_str(budget_crores).unwrap(),
            collection_crores: Decimal::from_str(collection_crores).unwrap(),
            verdict: verdict.to_owned(),
        };
        Ok(movie)
    }
}
