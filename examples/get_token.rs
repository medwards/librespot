use std::env;
use std::str::FromStr;

use librespot::core::authentication::Credentials;
use librespot::core::config::SessionConfig;
use librespot::core::keymaster;
use librespot::core::session::Session;
use std::path::PathBuf;

#[tokio::main]
async fn main() {
    let session_config = SessionConfig::default();

    let args: Vec<_> = env::args().collect();
    if args.len() != 4 {
        eprintln!(
            "Usage: {} CREDENTIALS_CACHE_PATH CLIENT_ID COMMA_SEPARATED_SCOPES",
            args[0]
        );
        return;
    }
    let cache =
        librespot::core::cache::Cache::new(Some(PathBuf::from_str(&args[1]).unwrap()), None, None)
            .expect("Could not create librespot cache");

    println!("Connecting..");
    let credentials = cache.credentials().expect("No credentials present");
    let session = Session::connect(session_config, credentials, None)
        .await
        .unwrap();

    match keymaster::get_token(&session, &args[2], &args[3]).await {
        Ok(token) => println!("Token: {:#?}", token),
        Err(e) => println!(" Got error: {:?}", e),
    }
}
