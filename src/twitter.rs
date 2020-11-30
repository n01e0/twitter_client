extern crate oauthcli;
extern crate ureq;
extern crate url;
extern crate percent_encoding;
extern crate serde;
extern crate serde_json;
extern crate toml;

use std::fs;
use std::io::{Write, stdin};
use std::env;
use serde::{Deserialize, Serialize};
use oauthcli::*;
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

const FLAG: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`').add(b'+').add(b'!').add(b'?').add(b'"').add(b'\'').add(b'$').add(b'@').add(b'#').add(b'%').add(b'^').add(b'&').add(b'*').add(b'(').add(b')').add(b'=').add(b'+').add(b'[').add(b']').add(b'{').add(b'}').add(b';').add(b':').add(b'/').add(b',').add(b'`');

#[derive(Debug, Deserialize, Serialize)]
struct Config {
    api_key:        String,
    api_secret:     String,
    token:          String,
    token_secret:   String 
}

#[derive(Deserialize, Debug, Clone)] 
pub struct Errors {
    pub code:       usize,
    pub message:    String
}

#[derive(Deserialize, Debug, Clone)] 
pub struct ErrorResponse {
    pub errors: Vec<Errors>
}

#[derive(Deserialize, Debug, Clone)] 
pub struct User {
    pub name: String
}

#[derive(Deserialize, Debug, Clone)] 
pub struct TweetData {
    pub text: String,
    pub user: User
}
#[derive(Deserialize, Debug, Clone)] 
pub struct TweetResponse {
    pub tweet_data: TweetData
}

#[derive(Clone, Copy)]
pub enum Method {
    Get,
    Post
}

impl Method {
    pub fn to_str(&self) -> &str {
        match self {
            Method::Get => "GET",
            Method::Post => "POST",
        }
    }
}

pub struct Twitter {
    method:     Method,
    url:        url::Url,
    header:     oauthcli::OAuthAuthorizationHeader
}

pub struct TwitterBuilder {
    config:         Config,
    method:         Method,
    tweet_content:  Option<String>
}

pub fn configure(config_dir: String, filename: String) {
    let config_file = fs::File::create(&format!("{}/{}", &config_dir, &filename));
    let mut content = Config {
        api_key: String::new(),
        api_secret: String::new(),
        token: String::new(),
        token_secret: String::new()
    };
    match config_file {
        Ok(mut file) =>  {
            println!("Found config file");
            println!("Let's configure");
            let mut buf = String::new();
            println!("Input your API_KEY");
            stdin().read_line(&mut buf).unwrap();
            content.api_key = buf.trim().parse().ok().unwrap();
            buf.clear();
            println!("api_secret");
            stdin().read_line(&mut buf).unwrap();
            content.api_secret = buf.trim().parse().ok().unwrap(); 
            buf.clear();
            println!("token");
            stdin().read_line(&mut buf).unwrap();
            content.token = buf.trim().parse().ok().unwrap();
            buf.clear();
            println!("token_secret");
            stdin().read_line(&mut buf).unwrap();
            content.token_secret = buf.trim().parse().ok().unwrap();
            buf.clear();
            let toml = toml::to_string(&content).unwrap();
            write!(file, "{}", toml).unwrap();
        },
        Err(err) => match err.kind() {
            std::io::ErrorKind::NotFound => {
                println!("Not found");
                if let Err(err) = std::fs::create_dir(&config_dir) {
                    eprintln!("{}", err);
                } else {
                    configure(config_dir, filename);
                }
            },
            err => eprintln!("{}", std::io::Error::from(err)),
        }
    }
}

impl TwitterBuilder {
    pub fn new() -> TwitterBuilder {
        let config_dir = format!("{}/.twitter_client", env::var("HOME").unwrap());
        let config_filename = "config.toml".to_string();
        let path = format!("{}/{}", config_dir, config_filename);
        let config_data = match fs::read_to_string(&path) {
            Ok(data) => data,
            Err(_) =>  {
                configure(config_dir, config_filename);
                fs::read_to_string(&path).unwrap()
            },
        };
        TwitterBuilder {
            config: toml::from_str(&config_data).unwrap(),
            method: Method::Get,
            tweet_content: None
        }
    }

    pub fn get(mut self) -> TwitterBuilder {
        self.method = Method::Get;
        self
    }

    pub fn post(mut self, content: &str) -> TwitterBuilder {
        self.method = Method::Post;
        self.tweet_content = Some(format!("status={}",utf8_percent_encode(content, FLAG).to_string()));
        self
    }

    pub fn finish(&mut self) -> Twitter {
        let url = match self.method {
                Method::Get => {
                    url::Url::parse("https://api.twitter.com/1.1/statuses/home_timeline.json?count=1").unwrap()
                },
                Method::Post => {
                    url::Url::parse(
                        &format!("https://api.twitter.com/1.1/statuses/update.json?{}", &self.tweet_content.clone().unwrap()))
                        .unwrap()
                }
        };
        let header = OAuthAuthorizationHeaderBuilder::new(self.method.to_str(),
                &url, &self.config.api_key, &self.config.api_secret, SignatureMethod::HmacSha1)
                .token(&self.config.token, &self.config.token_secret)
                .finish_for_twitter();
        
        Twitter {
            url: url,
            method: self.method,
            header: header
        }
    }
}

#[derive(Clone)]
pub struct Response {
    pub tweet: Option<TweetResponse>,
    pub error: Option<ErrorResponse>
}

impl Twitter {
    pub fn call(self) -> Response {
        match self.method {
            Method::Get =>  {
                let resp = ureq::get(self.url.as_str())
                    .set("Authorization", &format!("OAuth {}", self.header.auth_param()))
                    .call();
                if resp.ok() {
                    return Response {
                        tweet: Some(serde_json::from_str(&resp.into_string().unwrap()).unwrap()),
                        error: None
                    }
                } else {
                    return Response {
                        tweet: None,
                        error: Some(serde_json::from_str(&resp.into_string().unwrap()).unwrap())
                    }
                }
            },

            Method::Post => {
                let resp = ureq::post(self.url.as_str())
                    .set("Authorization", &format!("OAuth {}", self.header.auth_param()))
                    .call();
                if resp.ok() {
                    return Response {
                        tweet: None,
                        error: None
                    }
                } else {
                    return Response {
                        tweet: None,
                        error: Some(
                            serde_json::from_str(
                                &resp.into_string().unwrap_or_else(|s|{eprintln!("{:#?}", s);panic!("can't into_string()")})
                            ).unwrap_or_else(|s| {
                                panic!("can't parse response. {:#?}", s);
                            }
                            ))
                    }
                }
            }
        }
    }
}

