extern crate oauthcli;
extern crate url;
extern crate ureq;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate toml;

use std::fs;
use serde_json::Result;
use oauthcli::*;
use url::form_urlencoded;

#[derive(Debug, Deserialize)]
struct Config {
    api_key: String,
    api_secret: String,
    token: String,
    token_secret: String 
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let conf: Config = toml::from_str(&fs::read_to_string("$HOME/.twitter_cli/config.toml").unwrap()).unwrap();
    if args.len() > 1 {
        tweet(&args[1], conf);
    } else {
        timeline(conf);
    }
}

fn tweet(content: &str, config: Config) {
    let formated_content = form_urlencoded::Serializer::new(String::new())
        .append_pair("status", &content)
        .finish();
    let url = url::Url::parse(&format!("https://api.twitter.com/1.1/statuses/update.json?{}", formated_content)).unwrap();
    let header =
        OAuthAuthorizationHeaderBuilder::new(
            "POST", &url, &config.api_key, &config.api_secret, SignatureMethod::HmacSha1)
        .token(&config.token, &config.token_secret)
        .finish_for_twitter();
    
    let resp = ureq::post(url.as_str())
                .set("Authorization", &format!("OAuth {}", header.auth_param()))
                .call();
    if resp.ok() {
        println!("Tweeted! \"{}\"", content);
    } else {
        eprintln!("Error! {}: {}", resp.status(), resp.status_text());
    }
}


fn timeline(config: Config) {
    #[derive(Serialize, Deserialize, Debug)] 
    struct User {
        name: String
    }
    
    #[derive(Serialize, Deserialize, Debug)] 
    struct TweetData {
        text: String,
        user: User
    }
    #[derive(Serialize, Deserialize, Debug)] 
    struct Json {
        tw: TweetData
    }

    let url = url::Url::parse("https://api.twitter.com/1.1/statuses/home_timeline.json?count=1").unwrap();
    let header =
        OAuthAuthorizationHeaderBuilder::new(
            "GET", &url, config.api_key, config.api_secret, SignatureMethod::HmacSha1)
        .token(&config.token, &config.token_secret)
        .finish_for_twitter();
    
    let resp = ureq::get(url.as_str())
                .set("Authorization", &format!("OAuth {}", header.auth_param()))
                .call();
    if resp.ok() {
        let top: Result<Json> = serde_json::from_str(&resp.into_string().unwrap());
        match top {
            Ok(top) => println!("{}: {}", &top.tw.user.name, &top.tw.text),
            Err(err) => eprintln!("{}", err),
        }
    } else {
        eprintln!("{}: {}", &resp.status(), &resp.status_text());
    }
}
