use serde::{Deserialize, Serialize};

use percent_encoding_rfc3986::percent_decode;

use crate::{
    entry::Entry,
    log::log_debug,
    node::NodeExt,
    procedures::get::{tree::Get, GetPrefixError, SearchResult},
    tree::Tree,
};

// This is parsed by serde as a single array of Entry
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(untagged)]
pub enum Output {
    Values(Vec<Entry>),
}

impl Default for Output {
    fn default() -> Self {
        let values = [];

        let values: Vec<Entry> = values.into();

        Output::Values(values)
    }
}

pub async fn get_entries(name: String, tree: Tree) -> Result<warp::reply::Json, warp::Rejection> {
    log_debug("-----------------------------------------------------------------");
    log_debug(&format!("Parsing {name}"));
    // Sometimes names have encoded characters like %%20 for space
    // This will try parsing these entries, if it fails,
    // It will just use the regular name
    let parsed_name: String = match percent_decode(name.as_bytes()) {
        Err(err) => {
            log_debug(&format!("Error parsing name {err}"));
            name
        }
        Ok(value) => match value.decode_utf8() {
            Err(err) => {
                log_debug(&format!("Error parsing name {err}"));
                name
            }
            Ok(value) => String::from(value),
        },
    };

    println!("Get entry {parsed_name} .");

    let results = match tree.get_top(&parsed_name) {
        Ok(value) => value,
        Err(err) => match err {
            GetPrefixError::NotFound(value) => {
                let message = format!("Prefix {value} not found");
                log_debug(&message);

                return Ok(warp::reply::json(&Output::default()));
            }
        },
    };

    Ok(warp::reply::json(&results))
}

pub async fn get_top_entries(tree: Tree) -> Result<warp::reply::Json, warp::Rejection> {
    get_entries("".to_string(), tree).await
}

impl From<SearchResult> for Output {
    fn from(search_result: SearchResult) -> Output {
        match search_result {
            SearchResult::Next(value) => {
                // This branch should never happen.
                // To help debug, collecting information about the node
                let info: String = match value.upgrade() {
                    None => "(Not found)".into(),
                    Some(pointer) => pointer.read().get_prefix().into(),
                };

                panic!("Attempted to convert a SearchResult::Next into an Output. Prefix: {info} .");
            }
            SearchResult::Success(values) => Output::Values(values),
        }
    }
}
