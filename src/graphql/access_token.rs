use crate::app_config;
use chrono::{offset::Utc, DateTime, Duration};
use graphql_client::GraphQLQuery;
use log::{debug, error, info};
use measure_time::info_time;
use serde::{Deserialize, Serialize};
use serde_json;

use reqwest;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.graphql",
    query_path = "src/graphql/access_token.graphql",
    response_derives = "Debug"
)]
struct GetAccessToken;

// Error to indicate the API call failed
pub struct TokenRetrieveError;
use std::fmt;

impl fmt::Display for TokenRetrieveError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Could not retrieve access token.") // user-facing output
    }
}

impl fmt::Debug for TokenRetrieveError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Could not retrieve access token.") // programmer-facing output
    }
}

// Error indicating we did not have a cached token
struct CacheError;

// This is what goes into our cache
#[derive(Serialize, Deserialize, Debug)]
struct CachedAccessToken {
    access_token: String,
    creation_time: DateTime<Utc>,
}

// Cache file. Should go to some $USER location later.
const CACHE_FILE: &str = "podchaser_access_token.json";

fn cache_token(token: String) -> Result<(), std::io::Error> {
    info_time!("Timing for cache_token():");

    let timestamp = Utc::now();

    info!(
        "Caching new access token with creation time {}",
        timestamp.format("%Y-%m-%d-%H:%M:%SZ")
    );

    let cached_object = CachedAccessToken {
        access_token: token,
        creation_time: timestamp,
    };

    let serialized = serde_json::to_string_pretty(&cached_object).unwrap();

    std::fs::write(CACHE_FILE, serialized)?;

    Ok(())
}

fn token_younger_than_1_year(token: &CachedAccessToken) -> bool {
    let now = Utc::now();
    let token_time = token.creation_time;

    /*
     * Make sure we renew slightly _before_ 1 year is over (podchaser sets the
     * token lifetime to 365 days on creation)
     */
    let token_plus_1_year = token_time.checked_add_signed(Duration::days(364)).unwrap();

    now < token_plus_1_year
}

fn read_cached_token() -> Result<String, CacheError> {
    if !std::path::Path::new(CACHE_FILE).exists() {
        return Err(CacheError);
    }

    let data = std::fs::read_to_string(CACHE_FILE);

    match data {
        Ok(s) => {
            //debug!("Read token JSON: {:?}", s);
            let token: CachedAccessToken = serde_json::from_str(&s).unwrap();

            if token_younger_than_1_year(&token) {
                Ok(token.access_token)
            } else {
                Err(CacheError)
            }
        }
        Err(e) => {
            error!("{:?}", e);
            Err(CacheError)
        }
    }
}

pub fn access_token(client: String, secret: String) -> Result<String, TokenRetrieveError> {
    info_time!("Timing for access_token():");

    let cached = read_cached_token();
    match cached {
        Ok(t) => {
            info!("Got a cached access token.");
            return Ok(t);
        }
        Err(CacheError) => {
            info!("No cached token. Need to get a new one.");
        }
    }

    let request_body = GetAccessToken::build_query(get_access_token::Variables { secret, client });

    let client = reqwest::blocking::Client::new();

    let res = client
        .post(app_config::PODCHASER_API_ENDPOINT)
        .json(&request_body)
        .send();

    match res {
        Ok(_) => {}
        Err(_) => return Err(TokenRetrieveError),
    };

    let response_body: graphql_client::Response<get_access_token::ResponseData> =
        res.unwrap().json().unwrap();

    match response_body.data.unwrap().request_access_token {
        Some(o) => {
            cache_token(o.access_token.clone());
            Ok(o.access_token)
        }
        None => {
            for e in response_body.errors.unwrap() {
                error!("GraphQL Error: {:?}", e);
            }
            Err(TokenRetrieveError)
        }
    }
}
