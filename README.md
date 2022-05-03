# transmission-rss
A simple binary that reads a config file with a list of rss torrent items and adds them
to transmission.

```
$ transmission-rss
USAGE:
    transmission-rss --config <CONFIG>

OPTIONS:
    -c, --config <CONFIG>    Path to the config file
    -h, --help               Print help information
    -V, --version            Print version information
                                                        
```

### Getting started

```
$ cargo install transmission-rss
$ transmission-rss -c config.toml
```

### Config file

Example of `config.toml`:

```toml
[persistence]
path = "/path/to/db/folder"

[transmission]
url = "http://myserver/transmission/rpc"
username = "myusername"
password = "mypassword"

[notification.telegram]
bot_token = 123123:your_token
chat_id = 926175310

[[rss_list]]
title = "My List"
url = "https://someweb.site/rss.xml"
filters = ["1080p"]
download_dir = "/downloads/my_folder"
```
