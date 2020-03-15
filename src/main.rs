mod twitter;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.len() {
        1 => {
            let resp = twitter::TwitterBuilder::new().get().finish().call();
            if let Some(err_resp) = resp.error {
                for error in err_resp.errors {
                    eprintln!("{}: {}", &error.code, &error.message);
                }
            } else {
                let tw = resp.tweet.unwrap().tweet_data;
                println!("{}: {}", &tw.user.name, &tw.text);
            }
        },

        2 => {
            let resp = twitter::TwitterBuilder::new().post(&args[1]).finish().call();
            if let Some(err_resp) = resp.error {
                for error in err_resp.errors {
                    eprintln!("{}: {}", &error.code, &error.message);
                }
            } else {
                println!("Tweeted!: \"{}\"", args[1]);
            }
        },
        
        _ => usage(args[0].clone()),
    }
}

fn usage(name: String) {
    println!("Usage: {} <content>", name);
}
