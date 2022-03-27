use crate::app_config;
use graphql_client::GraphQLQuery;
use log::{debug, info};
use std::error::Error;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.graphql",
    query_path = "src/graphql/get_api_version.graphql",
    response_derives = "Debug"
)]
struct GetAPIVersion;

pub fn query_api_version() -> Result<(), Box<dyn Error>> {
    let request_body = GetAPIVersion::build_query(get_api_version::Variables {});

    let client = reqwest::blocking::Client::new();
    let res = client
        .post(app_config::PODCHASER_API_ENDPOINT)
        .json(&request_body)
        .send()?;

    debug!("### Response headers:");
    for (k, v) in res.headers().iter() {
        debug!("    {:?} -> {:?}", k, v);
    }

    let response_body: graphql_client::Response<get_api_version::ResponseData> = res.json()?;

    info!("### Response body: {:#?}", response_body);

    Ok(())
}
