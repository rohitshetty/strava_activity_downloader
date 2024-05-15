use clap::{Parser, Subcommand};
use reqwest::{self, header::AUTHORIZATION};
use confy;
use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct StravaConfig {
    athlete_id: String,
    access_code: String,
}

impl ::std::default::Default for StravaConfig {
    fn default() -> Self { Self { athlete_id: "xxxxx".into(), access_code: "xxxxx".into() } }
}

#[derive(Debug, Parser)]
#[command(name = "strava_activity_downloader")]
#[command(about = "View and download strava activity", long_about=None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(about = "List strava activities")]
    List,
}

#[tokio::main]
async fn main() ->Result<(), confy::ConfyError> {
    let cfg:StravaConfig = confy::load_path("./config.toml")?;
    println!("config: {:?}", cfg);
    let args = Cli::parse();

    match args.command {
        Commands::List => {
            show_activities(&cfg).await
        },
    }

    Ok(())
}

// Define a function that does the listing
async fn show_activities(config: &StravaConfig) {
    println!("Download and show the activities");
    let client = reqwest::Client::new();
    let url = format!("https://www.strava.com/api/v3/athletes/{}/activities", config.athlete_id);
    let authorization = format!("Bearer {}", config.access_code);
    let result = client.get(url)
    .header(AUTHORIZATION, authorization)
    .send()
    .await
    .unwrap()
    .text()
    .await;

    println!("Result: {:?}", result)

}

// fn get_activities() {
// Here Strava specific call is made
// }
