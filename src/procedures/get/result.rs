use std::sync::Weak;

use parking_lot::RwLock;

use crate::{entry::Entry, node::Node};

pub enum SearchResult {
    Success(Vec<Entry>),
    Next(Weak<RwLock<Node>>),
}
