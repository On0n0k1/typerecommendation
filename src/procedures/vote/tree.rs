use std::sync::{Arc, Weak};

use parking_lot::RwLock;

use crate::{
    log::log_debug,
    node::Node,
    procedures::vote::{node::Vote as NodeVote, VoteResult},
};

pub use crate::tree::TreeExt;

/// Contains all Tree logic for incrementing 'times' related to a given name.
pub trait Vote {
    /// Check Prefix Tree for given name.
    ///
    /// If found, increment 'times' and return Entry.
    /// If Not Found, returns VoteResult::NotFound.
    fn vote(&self, name: &str) -> VoteResult
    where
        Self: TreeExt,
    {
        log_debug("------------------------");
        let mut counter: usize = 0;

        let mut next: Weak<RwLock<Node>> = Arc::downgrade(self.get_node());

        log_debug("Starting loop post_entry");
        loop {
            log_debug("------------------------");
            next = match next
                .upgrade()
                .expect("Tried to unlock Node but got None .")
                .write()
                .vote(name, &mut counter)
            {
                VoteResult::Next(lock) => {
                    log_debug("post_entry Next");
                    lock
                }
                VoteResult::NotFound => {
                    log_debug("post_entry Not Found");
                    return VoteResult::NotFound;
                }
                success => return success,
            };
        }
    }
}
