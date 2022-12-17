//! Contains the source code for the Prefix Tree
//!
//!
//!
//! 

mod ext;

use std::sync::Arc;

use parking_lot::RwLock;

use crate::{
    node::Node,
    procedures::{
        get::tree::Get,
        load::{tree::Load, LoadError},
        vote::tree::Vote,
    },
};

pub use crate::tree::ext::TreeExt;

pub type Counter = usize;

// A prefix tree for managing data.
pub struct Tree {
    // The first node
    node: Arc<RwLock<Node>>,
}

// Cloning the tree will create another atomic pointer to the same node.
impl Clone for Tree {
    fn clone(&self) -> Self {
        let node = Arc::clone(&self.node);
        Self { node }
    }
}

impl TreeExt for Tree {
    fn get_node<'a>(&'a self) -> &Arc<RwLock<Node>> {
        &self.node
    }
}

impl Get for Tree {}

impl Vote for Tree {}

impl Load for Tree {}

impl Tree {
    /// Creates an instance of Tree and load entries from '/names.json' .
    pub async fn new(suggestions: usize) -> Result<Self, LoadError> {
        let tree: Tree = Tree::new_empty(suggestions).await;
        tree.load()?;

        Ok( tree )
    }

    pub async fn new_empty(suggestions: usize) -> Self {
        let node: Arc<RwLock<Node>> = Node::new(None, "".into(), 0, suggestions);
        Tree { node }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     use crate::{
//         endpoints::typeahead::prefix::get::Output as GetNamesOutput,
//         entry::Entry,
//         env::{EnvVars, SuggestionNumber},
//     };

//     async fn new_tree() -> Tree {
//         let env_vars = match EnvVars::new() {
//             Ok(value) => value,
//             Err(err) => panic!("Environment Variables Error: {}", err),
//         };

//         let suggestions: SuggestionNumber = env_vars.suggestion_number;

//         match Tree::new(suggestions).await {
//             Ok(value) => value,
//             Err(err) => panic!("Error Loading Tree: {}", err),
//         }
//     }

//     #[tokio::test]
//     async fn get_all() {
//         let tree: Tree = new_tree().await;

//         let all: Vec<Entry> = match tree.get_top("") {
//             Ok(GetNamesOutput::Values(value)) => value,
//             Err(err) => panic!("Tree::get_top_prefix('') return Error:{err}"),
//         };

//         let expected: Vec<Entry> = Vec::from([
//             Entry::new("Fidela", 999),
//             Entry::new("Gert", 999),
//             Entry::new("Guinna", 999),
//             Entry::new("Jenica", 999),
//             Entry::new("Merle", 999),
//             Entry::new("Adora", 998),
//             Entry::new("Aurea", 998),
//             Entry::new("Ginelle", 998),
//             Entry::new("Merilee", 998),
//             Entry::new("Miof Mela", 998),
//         ]);

//         for i in 0..all.len() {
//             println!("{} == {}", all[i].get_times(), expected[i].get_times());
//             assert_eq!(all[i].get_times(), expected[i].get_times());
//             println!("{} == {}", all[i].get_name(), expected[i].get_name());
//             assert_eq!(all[i].get_name(), expected[i].get_name());
//         }
//     }
// }
