mod twitter;

#[macro_use]
extern crate clap;
extern crate colored;

use colored::*;
use std::io::Read;

fn main() {
    let args = clap_app!(twitter_client =>
            (version:   crate_version!())
            (author:    crate_authors!())
            (about:     crate_description!())
            (@arg input: -i --input "read from stdin")
            (@arg content: "Tweet content")
        ).get_matches();

    if let Some(content) = args.value_of("content") {
        let resp = twitter::TwitterBuilder::new().post(content).finish().call();
        if let Some(err_resp) = resp.error {
            for error in err_resp.errors {
                eprintln!("{}", format!("{}: {}", &error.code, &error.message).red());
            }
        } else {
            println!("Tweeted!: \"{}\"", content);
        }
    } else if args.is_present("input") {
        let mut content = String::new();
        std::io::stdin().read_to_string(&mut content).unwrap();
        let resp = twitter::TwitterBuilder::new().post(&content).finish().call();
        if let Some(err_resp) = resp.error {
            for error in err_resp.errors {
                eprintln!("{}", format!("{}: {}", &error.code, &error.message).red());
            }
        } else {
            println!("Tweeted!: \"{}\"", content);
        }

    } else {
        let resp = twitter::TwitterBuilder::new().get().finish().call();
        if let Some(err_resp) = resp.error {
            for error in err_resp.errors {
                eprintln!("{}", format!("{}: {}", &error.code, &error.message).red());
            }
        } else {
            let tw = resp.tweet.unwrap().tweet_data;
            println!("{}: {}", &tw.user.name, &tw.text);
        }
    }
}
