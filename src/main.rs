use dirs;
use oauth_client::Token;
use serde_derive::Deserialize;
use std::error::Error;
use std::io;
use std::io::Read;
use std::path::PathBuf;
use structopt::*;
use twitter_api as t;

#[derive(Clone, Debug, StructOpt)]
#[structopt(name = "twirl")]
struct Options {
    #[structopt(short = "l", long = "credentials-location", parse(from_str))]
    credentials_location: Option<PathBuf>,

    #[structopt()]
    tweet: Option<String>,
}

#[derive(Deserialize, Debug)]
struct TwitterCredentials<'a> {
    consumer_key: &'a str,
    consumer_secret: &'a str,
    access_token: &'a str,
    access_token_secret: &'a str,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let options = Options::from_args();

    let credentials_location = if let Some(credentials_location) = options.credentials_location {
        credentials_location
    } else {
        let homedir = dirs::home_dir().ok_or("Could not get home dir")?;
        let mut credentials_location = PathBuf::new();
        credentials_location.push(homedir);
        credentials_location.push(".twitter_credentials.json");
        credentials_location
    };

    let credentials_string = std::fs::read_to_string(credentials_location)?;
    let credentials: TwitterCredentials = serde_json::from_str(&credentials_string)?;

    let consumer_token = Token::new(credentials.consumer_key, credentials.consumer_secret);
    let access_token = Token::new(credentials.access_token, credentials.access_token_secret);

    let tweet = if let Some(tweet_arg) = options.tweet {
        tweet_arg
    } else {
        let mut tweet_buf = String::new();
        io::stdin().read_to_string(&mut tweet_buf)?;
        tweet_buf
    };

    if tweet.len() <= 280 {
        match t::update_status(&consumer_token, &access_token, &tweet).await {
            Ok(_) => Ok(()),
            Err(err) => Err(Box::new(err.compat()) as Box<dyn Error>),
        }
    } else {
        let input_error = io::Error::new(
            io::ErrorKind::InvalidInput,
            "Tweet is longer than 280 characters!",
        );
        Err(Box::new(input_error) as Box<dyn Error>)
    }
}
