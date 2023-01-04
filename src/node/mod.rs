use std::collections::HashMap;
use std::sync::{Arc, Weak};

use parking_lot::RwLock;

mod ext;

use crate::{
    entry::Entry,
    log::log_debug,
    procedures::{get::node::Get, load::node::Load, vote::node::Vote},
};

pub use crate::node::ext::NodeExt;

// Weak: It is similar to the atomic Arc pointer, but it doesn't own the pointer.
//
// Which means that, when Arc gets removed from memory,
// the weak pointer will avoid a memory leak by returning None instead.
//
// Therefore parents have Arc pointers of their children,
// but children have Weak pointers of themselves and their parents.

/// A Node belonging to the Prefix Tree.
///
/// Contains an Entry, Reference to parent Node, Reference to itself, and Reference to child Nodes.
pub struct Node {
    parent: Option<Weak<RwLock<Self>>>,
    entry: Entry,
    children: HashMap<String, Arc<RwLock<Self>>>,

    // A reference to itself, used for creating children
    this_lock: Option<Weak<RwLock<Self>>>,

    // Number of suggestions for each request
    suggestions: usize,
}

impl Load for Node {}

impl Get for Node {}

impl Vote for Node {}

impl Node {
    pub fn new(
        parent: Option<Weak<RwLock<Self>>>,
        prefix: String,
        times: u64,
        suggestions: usize,
    ) -> Arc<RwLock<Self>> {
        let entry = Entry::new(prefix, times);
        let this = Self {
            parent,
            entry,
            children: HashMap::new(),
            this_lock: None,
            suggestions,
        };

        let this = Arc::new(RwLock::new(this));
        let reference = Arc::downgrade(&this);

        this.write().set_this_lock(reference);

        this
    }
}
#[cfg(not(test))]
impl Drop for Node {
    fn drop(&mut self) {
        log_debug(&format!("Shutting down Node {} .", self.entry.get_name()))
    }
}

impl NodeExt for Node {
    fn get_suggestions(&self) -> usize {
        self.suggestions
    }

    fn get_prefix(&self) -> &str {
        self.get_entry().get_name()
    }

    fn get_times(&self) -> &u64 {
        self.entry.get_times()
    }

    fn get_times_mut(&mut self) -> &mut u64 {
        self.entry.get_times_mut()
    }

    fn get_entry(&self) -> &Entry {
        &self.entry
    }

    fn get_parent(&self) -> Option<Weak<RwLock<Node>>> {
        log_debug("\nFunction crate::node::NodeExt::get_parent...\n");
        log_debug(&format!(
            "Node prefix: {}, times: {} is_none: {} .",
            &self.entry.get_name(),
            self.entry.get_times(),
            self.parent.is_none()
        ));

        self.parent.clone()
    }

    fn get_children(&self) -> &HashMap<String, Arc<RwLock<Self>>> {
        &self.children
    }

    fn get_lock(&self) -> Weak<RwLock<Node>> {
        match &self.this_lock {
            None => panic!(
                "Tried to get a lock but got None. Prefix: {}, times: {}, children {}",
                self.entry.get_name(),
                self.entry.get_times(),
                self.children.len()
            ),
            Some(value) => value.clone(),
        }
    }

    fn is_name(&self) -> bool {
        *self.entry.get_times() > 0
    }

    fn set_this_lock(&mut self, lock: Weak<RwLock<Node>>) {
        self.this_lock = Some(lock);
    }

    fn next_child(&self, character: &str) -> Option<Weak<RwLock<Node>>> {
        debug_assert_eq!(
            character.len(),
            1,
            "Expected a single character, got {} with {}.",
            character,
            character.len(),
        );

        let lowercase_character = character.to_ascii_lowercase();

        self.children.get(&lowercase_character).map(Arc::downgrade)
    }

    fn next_child_create(&mut self, character: &str) -> Weak<RwLock<Node>> {
        debug_assert_eq!(
            character.len(),
            1,
            "Expected a single character, got {} with {}.",
            character,
            character.len(),
        );

        let lowercase_character = character.to_string().to_ascii_lowercase();

        let child: Weak<RwLock<Node>> = match self.children.get(&lowercase_character) {
            Some(value) => Arc::downgrade(value),
            None => {
                // Creating and assigning child
                let parent = Some(self.get_lock());
                let mut prefix: String = self.entry.get_name().to_string();
                prefix.push_str(character);
                let times = 0;
                let suggestions = self.suggestions;

                let child = Self::new(parent, prefix, times, suggestions);

                let next_child = Arc::downgrade(&child);

                self.children.insert(lowercase_character, child);

                next_child
            }
        };

        child
    }
}
