use rss::{Channel, Item};
use std::error::Error;
use std::fs;
use transmission_rpc::types::{BasicAuth, RpcResponse, TorrentAddArgs, TorrentAdded};
use transmission_rpc::TransClient;
use transmission_rss::config::{Config, RssList};

async fn process_feed(
    item: RssList,
    cfg: Config,
) -> Result<Vec<String>, Box<dyn Error + Send + Sync>> {
    // Fetch the url
    let content = reqwest::get(item.url).await?.bytes().await?;
    let channel = Channel::read_from(&content[..])?;

    // Creates a new connection
    let basic_auth = BasicAuth {
        user: cfg.transmission.username.clone(),
        password: cfg.transmission.password.clone(),
    };
    let client = TransClient::with_auth(&cfg.transmission.url, basic_auth);

    // Filters the results
    let results: Vec<&Item> = channel
        .items()
        .into_iter()
        .filter(|it| {
            let mut found = false;

            for filter in item.filters.clone() {
                if it.title().unwrap_or_default().contains(&filter) {
                    found = true;
                }
            }

            found
        })
        .collect();

    let mut added: Vec<String> = vec![];
    for result in results {
        // Add the torrent into transmission
        let add: TorrentAddArgs = TorrentAddArgs {
            filename: Some(result.link().unwrap_or_default().to_string()),
            download_dir: Some(item.download_dir.clone()),
            ..TorrentAddArgs::default()
        };
        let res: RpcResponse<TorrentAdded> = client.torrent_add(add).await?;
        if res.is_ok() {
            added.push(result.title().unwrap_or_default().to_string());
        }
    }

    Ok(added)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read initial config file
    let file = fs::read_to_string("config.toml").unwrap();
    let cfg: Config = toml::from_str(&file).unwrap();

    let items: Vec<_> = cfg
        .clone()
        .rss_list
        .into_iter()
        .map(|it| process_feed(it, cfg.clone()))
        .collect();

    for item in items {
        match item.await {
            Ok(list) => {
                println!("{:?} items processed", list.len());
            }
            Err(err) => {
                println!("{:?}", err);
            }
        }
    }

    Ok(())
}
