use oauth2::basic::BasicClient;
use oauth2::{AuthUrl, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope, TokenUrl};
use std::env;
extern crate flexi_logger;
use log::{debug, info};
use measure_time::info_time;

pub mod app_config;
pub mod graphql;

use graphql::access_token::access_token;

#[allow(dead_code)] // TODO remove at some point?
fn generate_oauth2_url(client_id: String, client_secret: String) {
    let auth_url =
        AuthUrl::new(app_config::PODCHASER_AUTH_ENDPOINT.to_string()).expect("Invalid auth URL");
    let token_url =
        TokenUrl::new(app_config::PODCHASER_API_ENDPOINT.to_string()).expect("Invalid token URL");

    /* This should redirect to localhost so we could set up a local TCP server to get the redirect request and then parse the auth code.
    However, podchaser does not support this right now :( */
    let redirect_url = RedirectUrl::new(app_config::PODCHASER_REDIRECT_URI.to_string())
        .expect("Invalid redirect URL");

    let client = BasicClient::new(
        ClientId::new(client_id),
        Some(ClientSecret::new(client_secret)),
        auth_url,
        Some(token_url),
    )
    // Set the URL the user will be redirected to after the authorization process.
    .set_redirect_uri(redirect_url);

    // Generate the full authorization URL. For now, we just request ALL the scopes podchaser supports.
    let (goto_url, _csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        // Set the desired scopes.
        .add_scope(Scope::new("edit_ratings".to_string()))
        .add_scope(Scope::new("edit_reviews".to_string()))
        .add_scope(Scope::new("edit_follows".to_string()))
        .add_scope(Scope::new("edit_lists".to_string()))
        .add_scope(Scope::new("edit_listens".to_string()))
        .url();

    println!("Please open: {}", goto_url);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    flexi_logger::Logger::try_with_str("debug")
        .unwrap()
        .log_to_stderr()
        .format(flexi_logger::colored_detailed_format)
        .start()
        .unwrap();

    info_time!("main()");

    info!("BDPod starting up.");

    /*
     * Client ID and secret are configured via https://www.podchaser.com/profile/settings/api and need to be provided
     * at application launch. (Actually, those are app-specific, so if we released this, we'd likely hard-code them.)
     */
    let client_id = env::var("CLIENT_ID").expect("Missing CLIENT_ID");
    let client_secret = env::var("CLIENT_SECRET").expect("Missing CLIENT_SECRET");

    debug!("Using client ID:     {}", client_id);
    debug!("Using client secret: {}", client_secret);

    /*
     * Some mutating queries require OAuth authentication with PodChaser.
     *
     * The authorization token is created as follows (https://api-docs.podchaser.com/docs/guides/oauth-guide/):
     *   - Post an OAUTH request to https://www.podchaser.com/do-auth, add the right scopes you want to request
     *     authorization for. For convenience, the generate_oauth2_url() function above generates the right URL
     *     to call.
     *   - Successful OAuth authorization (via your browser) will redirect you to a redirect URI, which gets an extra
     *     CODE parameter, which is the authorization token to be used for this app+user combination.
     *     - Some OAuth2 examples, such as https://github.com/ramosbugs/oauth2-rs/blob/main/examples/wunderlist.rs
     *       use a TCP server on localhost as the redirect URI, so that they can grab the auth token directly.
     *     - Unfortunately, Podchaser's API does not allow us to set http://localhost as the redirect URI because
     *       this fails their URL validation. Hence, we need to redirect elsewhere.
     *   - Once successfully redirected, check the full URL (not only the part your browser shows you). For Podchaser
     *     it will contain someting like `/code=XYZ`. XYZ is the authorization token we're looking for and needs to
     *     be passed to the mutating call.
     */
    //generate_oauth2_url(client_id.clone(), client_secret.clone());
    //let mut token = String::new();
    //io::stdin().read_line(&mut token)?;
    // TODO: find a way that doesn't require copy-pasting magic values on the command line

    //query_api_version()?;
    debug!("Getting access token.");
    let access_token = access_token(client_id, client_secret)
        .expect("Unable to get an access token for podchaser.");
    debug!("Access token: {}", access_token);

    info!("The end.");

    Ok(())
}
