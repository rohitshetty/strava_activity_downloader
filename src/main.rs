use clap::{Parser, Subcommand};

/*
#[] here is called as attributes. This is used for various purposes.
One of them is for automatic implementation of traits - I think of this as method inheritance
*/

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

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::List => show_activities(),
    }
}

// Define a function that does the listing
fn show_activities() {
    println!("Download and show the activities")
    // Here we first make the api call and sort it chronologically
    // And return the json / whatever the data format is
}

// fn get_activities() {
// Here Strava specific call is made
// }
