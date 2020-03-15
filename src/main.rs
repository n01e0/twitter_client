mod twitter;

#[macro_use]
extern crate clap;

use clap::{App, Arg};

fn main() {
    let app = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(Arg::with_name("content")
             .help("Tweet content")
        );

    let args = app.get_matches();
    if let Some(content) = args.value_of("content") {
        let resp = twitter::TwitterBuilder::new().post(content).finish().call();
        if let Some(err_resp) = resp.error {
            for error in err_resp.errors {
                eprintln!("{}: {}", &error.code, &error.message);
            }
        } else {
            println!("Tweeted!: \"{}\"", content);
        }
    } else {
        let resp = twitter::TwitterBuilder::new().get().finish().call();
        if let Some(err_resp) = resp.error {
            for error in err_resp.errors {
                eprintln!("{}: {}", &error.code, &error.message);
            }
        } else {
            let tw = resp.tweet.unwrap().tweet_data;
            println!("{}: {}", &tw.user.name, &tw.text);
        }
    }
}
