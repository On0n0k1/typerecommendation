use std::sync::Weak;

use parking_lot::RwLock;

use crate::{entry::Entry, node::Node};

/// Result for retrieving top recommendations from a given prefix.
pub enum SearchResult {
    Success(Vec<Entry>),
    Next(Weak<RwLock<Node>>),
}
