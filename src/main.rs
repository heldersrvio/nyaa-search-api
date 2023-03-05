#[macro_use]
extern crate gotham_derive;
#[macro_use]
extern crate serde_derive;

use gotham::hyper::{Body, Response, StatusCode};
use gotham::helpers::http::response::create_response;
use gotham::handler::{HandlerError, HandlerFuture};
use gotham::router::{builder::*, Router};
use gotham::middleware::{Middleware, NewMiddleware};
use gotham::state::{FromState, State};
use tokio::task;
use serde::Serialize;
use serde_json::json;
use select::document::Document;
use select::predicate::{Attr, Class, Name};
use std::convert::TryInto;

#[derive(Serialize, Debug)]
struct Torrent {
    category: String,
    name: String,
    download_link: String,
    magnet_link: String,
    size: String,
    date_uploaded: String,
    seeders: i32,
    leechers: i32,
    completed_downloads: i32,
}

#[derive(Serialize, Debug)]
struct Error {
    message: String,
}

#[derive(Deserialize, StateData, StaticResponseExtender)]
struct QueryStringExtractor {
    query: String,
}

async fn search(search: &str) -> Result<Vec<Torrent>, &'static str> {
    let base_url = "https://nyaa.si?q=";
    let search_url = format!("{}{}", base_url, search);
    let body = reqwest::get(&search_url)
        .await
        .map_err(|_| "Failed to make request")?
        .text()
        .await
        .map_err(|_| "Failed to get response body")?;
    let document = Document::from(body.as_str());
    let mut torrents = vec![];

    for tbody in document.find(Name("tbody")).take(1) {
        for node in tbody.find(Name("tr")).take(10) {
            let mut td_nodes = node.find(Name("td"));
            let category_node = td_nodes
                .next()
                .ok_or("Failed to retrieve category node")?;
            let name_node = td_nodes
                .next()
                .ok_or("Failed to retrieve name node")?;
            let download_node = td_nodes
                .next()
                .ok_or("Failed to retrieve download node")?;
            let size_node = td_nodes
                .next()
                .ok_or("Failed to retrieve size node")?;
            let date_node = td_nodes
                .next()
                .ok_or("Failed to retrieve date node")?;
            let seeders_node = td_nodes
                .next()
                .ok_or("Failed to retrieve seeders node")?;
            let leechers_node = td_nodes
                .next()
                .ok_or("Failed to retrieve leechers node")?;
            let completed_node = td_nodes
                .next()
                .ok_or("Failed to retrieve completed node")?;

            let category = category_node
                .find(Name("a"))
                .next()
                .ok_or("Failed to retrieve category title")?
                .attr("title") 
                .ok_or("Failed to retrieve category title")?
                .to_string();
           
            let name = name_node
                .find(Name("a"))
                .next()
                .ok_or("Failed to retrieve name title")?
                .attr("title")
                .ok_or("Failed to retrieve name title")?
                .to_string();

            let download_link = download_node
                .find(Name("a"))
                .next()
                .ok_or("Failed to retrieve download link")?
                .attr("href")
                .ok_or("Failed to retrieve download link")?
                .to_string();

            let magnet_link = download_node
                .find(Name("a"))
                .nth(1)
                .ok_or("Failed to retrieve magnet link")?
                .attr("href")
                .ok_or("Failed to retrieve magnet link")?
                .to_string();

            let size = size_node
                .text()
                .to_string();

            /*let date_timestamp = date_node
                .attr("data-timestamp")
                .ok_or("Failed to retrieve data-timestamp")?;

            let date_uploaded = chrono::DateTime::from_utc(
                chrono::NaiveDateTime::from_timestamp(date_timestamp.try_into().unwrap(), 0),
                chrono::Utc,
            )
            .to_string();*/

            let seeders = seeders_node
                .text()
                .to_string()
                .parse::<i32>()
                .unwrap();

            let leechers = leechers_node
                .text()
                .to_string()
                .parse::<i32>()
                .unwrap();

            let completed_downloads = completed_node
                .text()
                .to_string()
                .parse::<i32>()
                .unwrap();

            torrents.push(Torrent {
                category,
                name,
                download_link,
                magnet_link,
                size,
                date_uploaded: "2022-03-01".to_string(),
                seeders,
                leechers,
                completed_downloads,
            });
        }
    }

    if torrents.is_empty() {
        return Err("No torrents found");
    }

    Ok(torrents)
}

async fn search_handler(mut state: State) -> Result<(State, Response<Body>), (State, HandlerError)> {
    let res = {
        let query_param = QueryStringExtractor::take_from(&mut state);
        let search_result = search(&query_param.query).await;
        match search_result {
            Ok(torrents) => create_response(
                &state,
                StatusCode::OK,
                mime::APPLICATION_JSON,
                serde_json::to_vec(&torrents).expect("serialized torrents"),
            ),
            Err(e) => create_response(
                &state,
                StatusCode::BAD_GATEWAY,
                mime::APPLICATION_JSON,
                serde_json::to_vec(&Error { message: e.to_string() }).expect("serialized error"),
            ) 
        }
    };
    Ok((state, res))
}

fn router() -> Router {
    build_simple_router(|route| {
        route
            .get("/search")
            .with_query_string_extractor::<QueryStringExtractor>()
            .to_async(search_handler);
    })
}

pub fn main() {
    let addr = "127.0.0.1:7878";
    println!("Listening for requests at http://{}", addr);
    gotham::start(addr, router())
}

