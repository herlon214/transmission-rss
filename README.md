# transmission-rss
A simple binary that reads a config file with a list of rss torrent items and adds them
to transmission.

Example of `config.toml`:

```toml
[persistence]
path = "/path/to/db/folder"

[transmission]
url = "http://myserver/transmission/rpc"
username = "myusername"
password = "mypassword"

[[rss_list]]
title = "Shijou"
url = "https://someweb.site/rss.xml"
filters = ["1080p"]
download_dir = "/downloads/my_folder"
```