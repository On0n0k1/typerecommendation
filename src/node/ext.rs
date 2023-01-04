use std::collections::HashMap;
use std::sync::{Arc, Weak};

use parking_lot::RwLock;

use crate::{entry::Entry, node::Node};

pub trait NodeExt {
    /// The max number of entries that can be returned through the GET request.
    fn get_suggestions(&self) -> usize;

    /// Prefix for a given Node.
    fn get_prefix(&self) -> &str;

    /// Reference to the number of times a Node was selected.
    fn get_times(&self) -> &u64;

    /// Mutable Reference to the number of times a Node was selected.
    fn get_times_mut(&mut self) -> &mut u64;

    /// Retrieving entry is the same as retrieving prefix and times together.
    fn get_entry(&self) -> &Entry;

    /// Atomic pointer to parent Node.
    fn get_parent(&self) -> Option<Weak<RwLock<Node>>>;

    /// Atomic pointers to child Nodes.
    fn get_children(&self) -> &HashMap<String, Arc<RwLock<Self>>>;

    /// Atomic pointer to this Node.
    fn get_lock(&self) -> Weak<RwLock<Node>>;

    /// If times is greater than 0, this is true.
    fn is_name(&self) -> bool;

    /// Used when Creating a Node. Set stored Atomic pointer to this same Node.
    fn set_this_lock(&mut self, lock: Weak<RwLock<Node>>);

    /// Returns a reference to the next node according to given character. Doesn't create a new Node.
    fn next_child(&self, character: &str) -> Option<Weak<RwLock<Node>>>;

    /// Returns a reference to the next node according to given character. If it doesn't exist, create a new Node and return it.
    fn next_child_create(&mut self, character: &str) -> Weak<RwLock<Node>>;
}
