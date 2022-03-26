/* Auto-generated Cynic query structs */

#[cynic::schema_for_derives(
    file = r#"src/schema.graphql"#,
    module = "schema",
)]
pub mod queries {
    use super::schema;

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Query")]
    pub struct APIVersion {
        pub api_version: String,
    }

}

pub mod schema {
    cynic::use_schema!(r#"src/schema.graphql"#);
}