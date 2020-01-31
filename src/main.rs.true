extern crate oauthcli;
extern crate ureq;
extern crate url;
extern crate percent_encoding;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate toml;

use std::fs;
use std::env;
use serde_json::Result;
use oauthcli::*;
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

const FLAG: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`').add(b'+').add(b'!').add(b'?').add(b'"').add(b'\'').add(b'$').add(b'@').add(b'#').add(b'%').add(b'^').add(b'&').add(b'*').add(b'(').add(b')').add(b'=').add(b'+').add(b'[').add(b']').add(b'{').add(b'}').add(b';').add(b':').add(b'/').add(b',').add(b'`');

#[derive(Debug, Deserialize)]
struct Config {
    api_key: String,
    api_secret: String,
    token: String,
    token_secret: String 
}

#[derive(Deserialize, Debug)] 
struct Errors {
    code: usize,
    message: String
}
#[derive(Deserialize, Debug)] 
struct Response {
    errors: Vec<Errors>
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config_path = format!("{}/.twitter_cli/config.toml", env::var("HOME").unwrap());
    match fs::read_to_string(&config_path) {
        Ok(content) =>  {
            let conf: Config = toml::from_str(&content).unwrap();
            if args.len() > 1 {
                tweet(&args[1], conf);
            } else {
                timeline(conf);
            }
        },
        Err(err) => eprintln!("{}", err),
    }
}

fn tweet(content: &str, config: Config) {
    let formated_content = format!("status={}", utf8_percent_encode(content, FLAG).to_string());
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
        let err: Result<Response> = serde_json::from_str(&resp.into_string().unwrap());
        match err {
            Ok(response) => {
                eprintln!("Error code:{}\n\t{}", &response.errors[0].code, &response.errors[0].message);
                eprintln!("Send data = {}", formated_content);
            },
            Err(err) => eprintln!("{}", err),
        }
    }
}


fn timeline(config: Config) {
    #[derive(Deserialize, Debug)] 
    struct User {
        name: String
    }
    
    #[derive(Deserialize, Debug)] 
    struct TweetData {
        text: String,
        user: User
    }
    #[derive(Deserialize, Debug)] 
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
            Err(err) => eprintln!("Error: {}", err),
        }
    } else {
        eprintln!("{}: {}", &resp.status(), &resp.status_text());
    }
}
