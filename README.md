# twirl
no-hassle write-only tweeting from the command line

## use

By default Twirl will look for a `.twitter_credentials.json` file
in your home directory. That file should have a format like the one in
`.twitter_credentials.json.example`.

You should go to https://developer.twitter.com and register your own
app to get tokens for this purpose. This is an easy process
and should take less than 5 minutes if you already have a Twitter account.

```
clark$> twirl -h
twirl 0.1.0
Clark Kampfe <clark.kampfe@gmail.com>

USAGE:
    twirl [OPTIONS] [tweet]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -l, --credentials-location <credentials_location>

ARGS:
    <tweet>
```

You can also tweets into twirl via stdin,
like `$ echo "my great tweet!!!1111" | twirl`

## building

```
$ git clone git@github.com:ckampfe/twirl.git
$ cargo build --release
$ ln -s ~/code/twirl/target/release/twirl /usr/local/bin # or wherever on your path
$ touch ~/.twitter_credentials.json # elsewhere
# enter your twitter credentials like the ones in .twitter_credentials.json.example
```
