use crate::app_config;
use graphql_client::GraphQLQuery;
use log::error;
use measure_time::info_time;

use reqwest;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.graphql",
    query_path = "src/graphql/access_token.graphql",
    response_derives = "Debug"
)]
struct GetAccessToken;

pub struct TokenError;
use std::fmt;

impl fmt::Display for TokenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Could not retrieve access token.") // user-facing output
    }
}

impl fmt::Debug for TokenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Could not retrieve access token.") // programmer-facing output
    }
}

pub fn access_token(client: String, secret: String) -> Result<String, TokenError> {
    info_time!("Timing for access_token():");

    let request_body = GetAccessToken::build_query(get_access_token::Variables { secret, client });

    let client = reqwest::blocking::Client::new();

    let res = client
        .post(app_config::PODCHASER_API_ENDPOINT)
        .json(&request_body)
        .send();

    match res {
        Ok(_) => {}
        Err(_) => return Err(TokenError),
    };

    let response_body: graphql_client::Response<get_access_token::ResponseData> =
        res.unwrap().json().unwrap();

    match response_body.data.unwrap().request_access_token {
        Some(o) => Ok(o.access_token),
        None => {
            for e in response_body.errors.unwrap() {
                error!("GraphQL Error: {:?}", e);
            }
            Err(TokenError)
        }
    }
}
