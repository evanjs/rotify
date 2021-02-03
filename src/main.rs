extern crate env_logger;
extern crate rspotify;

#[macro_use]
extern crate clap;

use anyhow::{Error, Result};
use dotenv::dotenv;
use rspotify::client::SpotifyBuilder;
use rspotify::model::{Country, IncludeExternal, SearchResult::{Artists, Tracks}, SearchType};
use rspotify::oauth2::CredentialsBuilder;
use serde::Deserialize;

use clap::{App, Arg};

const DEFAULT_LIMIT: u32 = 1;
const DEFAULT_OFFSET: u32 = 0;
const DEFAULT_COUNTRY: Option<Country> = Some(Country::UnitedStates);
const DEFAULT_INCLUDE_EXTERNAL: Option<IncludeExternal> = None;
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    dotenv().ok();

    let matches = App::new("rotify")
        .version("0.1")
        .author("Evan Stoll <evanjsx@gmail.com>")
        .about("Queries spotify")
        .arg(Arg::with_name("input")
            .index(1)
            .required(true)
            .help("The query to perform"))
        .arg(Arg::with_name("search_type")
            .short("t")
            .long("type")
            .takes_value(true)
            .conflicts_with("track")
            .help("Defines the type of search to perform"))
        .get_matches();



    let query = value_t_or_exit!(matches.value_of("input"), String);
    let search_type = value_t!(matches.value_of("search_type"), String).unwrap_or_else(|e| e.exit());

    let creds = CredentialsBuilder::from_env().build().unwrap();

    let mut spotify = SpotifyBuilder::default()
        .credentials(creds)
        .build()
        .unwrap();

    let search_type = match search_type.as_str() {
        "artist" => SearchType::Artist,
        "track" => SearchType::Track,
        _ => panic!("Fuck you")
    };

    spotify.request_client_token().await?;

    println!("Searching for artist: {} ...", query);
    let search = spotify
        .search(
            &query,
            search_type,
            DEFAULT_LIMIT,
            DEFAULT_OFFSET,
            DEFAULT_COUNTRY,
            DEFAULT_INCLUDE_EXTERNAL,
        )
        .await?;
    match search {
        Tracks(t) => {
            if t.items.len() == 0 {
                println!("No results for track {}.  Exiting ...", query);
                std::process::exit(0);
            }
            
            let item = &t.items[0];
            let id = item.id.as_deref().unwrap();

            let features = spotify.track_features(id).await?;

            println!("Acoustics for {}: {:#?}", item.name, features);
            
        },
        Artists(a) => {
            if a.items.len() == 0 {
                println!("No results for artist {}.  Exiting ...", query);
                std::process::exit(0);
            }
            let item = &a.items[0];

            let id = &item.id;

            let tracks = spotify.artist_top_tracks(id, DEFAULT_COUNTRY).await?;

            println!("Top tracks from {}", query);
            for t in tracks {
                println!("{} from {}", t.name, t.album.name);
            }
        }
        _ => (),
    }

    Ok(())
}
