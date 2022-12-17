use std::collections::HashMap;
use std::sync::{Arc, Weak};

use parking_lot::RwLock;

use crate::{
    entry::Entry,
    log::log_debug,
    procedures::{get::node::Get, load::node::Load, vote::node::Vote},
};

// Weak: It is similar to the atomic Arc pointer, but it doesn't own the pointer.
//
// Which means that, when Arc gets removed from memory,
// the weak pointer will avoid a memory leak by returning None instead.
//
// Therefore parents have Arc pointers of their children,
// but children have Weak pointers of themselves and their parents.

pub struct Node {
    parent: Option<Weak<RwLock<Self>>>,
    prefix: String,
    times: u64,
    children: HashMap<String, Arc<RwLock<Self>>>,
    top: Vec<Entry>,

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
        let this = Self {
            parent,
            prefix,
            times,
            children: HashMap::new(),
            top: Vec::with_capacity(suggestions + 1),
            this_lock: None,
            suggestions,
        };

        let this = Arc::new(RwLock::new(this));
        let reference = Arc::downgrade(&this);

        this.write().set_this_lock(reference);

        this
    }
}

impl Drop for Node {
    fn drop(&mut self) {
        log_debug(&format!("Shutting down Node {} .", self.prefix))
    }
}

pub trait NodeExt {
    fn is_name(&self) -> bool;

    fn include_top(&mut self, entry: &Entry);

    fn get_prefix<'a>(&'a self) -> &'a str;

    fn get_prefix_len(&self) -> usize;

    fn get_times(&self) -> u64;

    // fn top_clone(&self) -> Vec<Entry>;
    fn get_top<'a>(&'a self) -> &'a Vec<Entry>; 

    fn update_top(&mut self, entry: &Entry) -> bool;

    fn get_top_len(&self) -> usize;

    /// Get the lowest entry of top recommendations.
    fn get_top_last(&self) -> Option<Entry>;

    fn get_suggestions(&self) -> usize;

    fn set_times(&mut self, times: u64);

    fn get_lock(&self) -> Weak<RwLock<Node>>;

    fn get_parent(&self) -> Option<Weak<RwLock<Node>>>;

    fn set_this_lock(&mut self, lock: Weak<RwLock<Node>>);

    fn next_child(&self, character: &str) -> Option<Weak<RwLock<Node>>>;

    fn next_child_create(&mut self, character: &str) -> Weak<RwLock<Node>>;

    /// Return the maximum size for the suggestion list.
    fn limit(&self) -> usize {
        let top_len = self.get_top_len();
        let suggestions = self.get_suggestions();

        if top_len < suggestions {
            return top_len;
        }

        suggestions
    }
}

impl NodeExt for Node {
    fn is_name(&self) -> bool {
        self.times > 0
    }

    fn include_top(&mut self, entry: &Entry) {
        let top = &mut self.top;
        let suggestions = &self.suggestions;

        // If there are few elements in the recommendation, just append the result.
        if top.len() < *suggestions {
            top.push(entry.clone());

            top.sort();

            return;
        }

        let lowest = top[top.len() - 1].get_times();

        // The entry is not high enough to be at the top recommendations
        if *lowest > *entry.get_times() {
            return;
        }

        // If lowest value is lower than current entry, push to the end of the list
        top.push(entry.clone());

        top.sort();

        // If over the limit, drop last element
        if top.len() > *suggestions {
            top.pop();
        }
    }

    fn get_prefix<'a>(&'a self) -> &'a str {
        // self.prefix.clone()
        &self.prefix[..]
    }

    fn get_prefix_len(&self) -> usize {
        self.prefix.len()
    }

    fn get_times(&self) -> u64 {
        self.times
    }

    fn get_top<'a>(&'a self) -> &'a Vec<Entry> {
        &self.top
    }

    fn update_top(&mut self, entry: &Entry) -> bool {
        let mut found = false;

        // If entry already exists, update times
        log_debug(&format!(
            "Updating top recommendations for nodes using name {} and times {}.",
            entry.get_name(), entry.get_times()
        ));

        for i in self.top.iter_mut() {
            if *i.get_times() + 1 == *entry.get_times() 
                && i.get_name().eq_ignore_ascii_case(entry.get_name()) {
                    log_debug(&format!(
                        "Found entry with name {} and times {}",
                        i.get_name(), i.get_times()
                    ));
                    found = true;
                    *i.get_times_mut() += 1;
                    break;
                
            }
        }

        if found {
            self.top.sort();
        }

        found
    }

    fn get_top_len(&self) -> usize {
        self.top.len()
    }

    fn get_top_last(&self) -> Option<Entry> {
        // match self.top.last() {
        //     None => None,
        //     Some(value) => Some(value.clone()),
        // }
        // self.top.last().map(|value| value.clone())
        self.top.last().cloned()
    }

    fn get_suggestions(&self) -> usize {
        self.suggestions
    }

    fn set_times(&mut self, times: u64) {
        self.times = times;
    }

    fn get_lock(&self) -> Weak<RwLock<Node>> {
        match &self.this_lock {
            None => panic!(
                "Tried to get a lock but got None. Prefix: {}, times: {}, children {}",
                self.prefix,
                self.times,
                self.children.len()
            ),
            Some(value) => value.clone(),
        }
    }

    fn get_parent(&self) -> Option<Weak<RwLock<Node>>> {
        log_debug("\nFunction crate::node::NodeExt::get_parent...\n");
        log_debug(&format!(
            "Node prefix: {}, times: {} is_none: {} .",
            &self.prefix,
            self.times,
            self.parent.is_none()
        ));

        self.parent.clone()
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

        // match &self.children.get(&lowercase_character) {
        //     None => None,
        //     Some(value) => Some(Arc::downgrade(value)),
        // }

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

        let lowercase_character = character.to_ascii_lowercase();

        let child: Weak<RwLock<Node>> = match self.children.get(&lowercase_character) {
            Some(value) => Arc::downgrade(value),
            None => {
                // Creating and assigning child
                let parent = Some(self.get_lock());
                let mut prefix = self.prefix.clone();
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
