#[doc(hidden)]
pub const CONNECTION_STRING: &str = "mongodb://127.0.0.1:27017";
#[doc(hidden)]
pub const DB_NAME: &str = "BoxOffice";
#[doc(hidden)]
pub const COLLECTION: &str = "Movies";
pub const PINGCOMMAND: &str = "ping";

pub const ID: &str = "_id";
pub const MOVIEID: &str = "MovieID";
pub const TITLE: &str = "Title";
pub const PRODUCTION_HOUSE: &str = "Production House";
pub const YEAR_OF_RELEASE: &str = "YearOfRelease";
pub const BUDGET_CRORES: &str = "BudgetCrores";
pub const COLLECTION_CRORES: &str = "CollectionCrores";
pub const VERDICT: &str = "Verdict";

pub const BUDGET_EQUALS_COLLECTION: &str = "AVERAGE";
pub const LOSS_PERCENTAGE_GREATER_THAN_40: &str = "DISASTER";
pub const LOSS_PERCENTAGE_GREATER_THAN_20_BUT_LESS_THAN_OR_EQUAL_TO_40: &str = "FLOP";
pub const LOSS_PERCENTAGE_GREATER_THAN_10_BUT_LESS_THAN_OR_EQUAL_TO_20: &str = "BELOW AVERAGE";
pub const LOSS_PERCENTAGE_GREATER_THAN_0_BUT_LESS_THAN_OR_EQUAL_TO_10: &str = "AVERAGE";

pub const PROFIT_PERCENTAGE_GREATER_THAN_0_BUT_LESS_THAN_OR_EQUAL_TO_10: &str = "AVERAGE";
pub const PROFIT_PERCENTAGE_GREATER_THAN_10_BUT_LESS_THAN_OR_EQUAL_TO_20: &str = "ABOVE AVERAGE";
pub const PROFIT_PERCENTAGE_GREATER_THAN_20_BUT_LESS_THAN_OR_EQUAL_TO_40: &str = "SEMI HIT";
pub const PROFIT_PERCENTAGE_GREATER_THAN_40_BUT_LESS_THAN_OR_EQUAL_TO_80: &str = "HIT";
pub const PROFIT_PERCENTAGE_GREATER_THAN_80_BUT_LESS_THAN_OR_EQUAL_TO_150: &str = "SUPER HIT";
pub const PROFIT_PERCENTAGE_GREATER_THAN_150_BUT_LESS_THAN_OR_EQUAL_TO_300: &str = "BLOCKBUSTER";
pub const PROFIT_PERCENTAGE_GREATER_THAN_300: &str = "ALL TIME BLOCKBUSTER";

pub const MINIMUM_YEAR_OF_RELEASE: u16 = 1989;
pub const MAXIMUM_YEAR_OF_RELEASE: u16 = 2023;
pub const ZERO: u8 = 0;

pub const DB_OPERATION_FAILED: &str = "Database operation failed";
pub const BUDGET_GREATER_THAN_ZERO_ERROR: &str = "Budget should be greater than zero";
pub const COLLECTION_GREATER_THAN_ZERO_ERROR: &str = "Collection should be greater than zero";
pub const MOVIEID_GREATER_THAN_ZERO_ERROR: &str = "Movie id should be greater than zero";
pub const YEAR_OF_RELEASE_BETWEEN_MIN_AND_MAX: &str =
    "Year Of Release should be between 1989 AND 2023";
