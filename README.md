# Twitter CLIent
## install
```sh
cargo install --git https://github.com/n01e0/twtter_client.git
```

You need write to `$HOME/.twitter_cli/config.toml`
```toml
api_key = "YOUR API KEY"
api_secret = "YOUR API SECRET KEY"
token = "YOUR ACCESS TOKEN"
token_secret = "YOUR ACCESS TOKEN SECRET"
```

## Usage
`twitter`

Print Home Timeline top tweet

`twitter "content"`

Tweet "content"
