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
    // This will try parsing these entries,
    // if it fails, just use the regular name
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
    get_entries("".into(), tree).await
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

                panic!(
                    "Attempted to convert a SearchResult::Next into an Output. Prefix: {info} ."
                );
            }
            SearchResult::Success(values) => Output::Values(values),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{entry::Entry, node::Node, procedures::get::SearchResult};

    use parking_lot::RwLock;
    use std::sync::{Arc, Weak};

    use std::cmp::Ordering;

    use crate::{
        procedures::{get::tree::Get, load::tree::Load},
        tree::Tree,
    };

    fn new_empty_next() -> SearchResult {
        // Node will be dropped
        let node: Arc<RwLock<Node>> = Node::new(None, "".into(), 0, 0);
        // This pointer will return None when accessed
        let lock: Weak<RwLock<Node>> = Arc::downgrade(&node);
        let next: SearchResult = SearchResult::Next(lock);

        next
    }

    fn new_next() -> (Arc<RwLock<Node>>, SearchResult) {
        let node: Arc<RwLock<Node>> = Node::new(None, "APrefix".into(), 0, 0);
        let lock: Weak<RwLock<Node>> = Arc::downgrade(&node);
        let next: SearchResult = SearchResult::Next(lock);

        (node, next)
    }

    mod request_conversion {
        use super::*;

        #[test]
        #[should_panic(
            expected = "Attempted to convert a SearchResult::Next into an Output. Prefix: (Not found) ."
        )]
        fn get_result_empty_next_error() {
            let next = new_empty_next();

            let _: Output = next.into();
        }

        #[test]
        #[should_panic(
            expected = "Attempted to convert a SearchResult::Next into an Output. Prefix: APrefix ."
        )]
        fn get_result_next_error() {
            let (_node, next) = new_next();

            let _: Output = next.into();
        }
    }

    const RECOMMENDATIONS: usize = 10;

    fn entries() -> Vec<Entry> {
        Vec::from([
            ("Alice", 300).into(),
            ("abc", 100).into(),
            ("aaron", 50).into(),
            ("loki", 40).into(),
            ("uthred", 30).into(),
            ("ANN", 10).into(),
            ("harald hardrada", 10).into(),
            ("ISabelLa", 10).into(),
            ("Shiro", 10).into(),
            ("sigvald", 10).into(),
            ("oliver", 5).into(),
            ("Olivia", 4).into(),
            ("Olly", 3).into(),
            ("Orianna", 2).into(),
            ("Orpheus", 1).into(),
        ])
    }

    async fn new_tree() -> Tree {
        let tree: Tree = Tree::new_empty(RECOMMENDATIONS).await;

        entries()
            .iter()
            .for_each(|entry| tree.include(entry.to_owned()).unwrap());

        tree
    }

    fn assert_entries(tree: &Tree, prefix: &str, expected: Vec<Entry>) {
        let Output::Values(results) = tree.get_top(prefix).unwrap();
        let mut valid: bool = expected.len().eq(&results.len());

        for i in 0..(expected.len()) {
            if expected[i].cmp(&results[i]).is_ne() {
                valid = false;
                break;
            }
        }

        if !valid {
            println!("Prefix: {prefix}");
            println!("Length(expected): {}", expected.len());
            println!("Length(Retrieved): {}", results.len());
            println!("\n\nAcquired/Expected:");
            for (index, result) in results.iter().enumerate() {
                let comparison: &str = match result.cmp(&expected[index]) {
                    Ordering::Equal => "=",
                    Ordering::Greater => ">",
                    Ordering::Less => "<",
                };

                println!("{result} {comparison} {}", expected[index]);
            }

            panic!("Failed comparison between entries.")
        }
    }

    #[tokio::test]
    async fn get_entries_success() {
        let tree: Tree = new_tree().await;

        let expected: Vec<Entry> = Vec::from(&entries()[0..RECOMMENDATIONS]);

        assert_entries(&tree, "", expected);

        let prefix: &str = "a";
        let expected: Vec<Entry> = [
            ("Alice", 300).into(),
            ("abc", 100).into(),
            ("aaron", 50).into(),
            ("ann", 10).into(),
        ]
        .to_vec();

        assert_entries(&tree, prefix, expected);

        let prefix: &str = "harald hardrada";
        let expected: Vec<Entry> = [("harald hardrada", 10).into()].to_vec();

        assert_entries(&tree, prefix, expected);

        let prefix: &str = "abc";
        let expected: Vec<Entry> = [("abc", 100).into()].to_vec();

        assert_entries(&tree, prefix, expected);

        let prefix: &str = "o";
        let expected: Vec<Entry> = [
            ("oliver", 5).into(),
            ("Olivia", 4).into(),
            ("Olly", 3).into(),
            ("Orianna", 2).into(),
            ("Orpheus", 1).into(),
        ]
        .to_vec();

        assert_entries(&tree, prefix, expected);

        let prefix: &str = "ol";
        let expected: Vec<Entry> = [
            ("oliver", 5).into(),
            ("Olivia", 4).into(),
            ("Olly", 3).into(),
        ]
        .to_vec();

        assert_entries(&tree, prefix, expected);

        let prefix: &str = "or";
        let expected: Vec<Entry> = [("Orianna", 2).into(), ("Orpheus", 1).into()].to_vec();

        assert_entries(&tree, prefix, expected);
    }
}
