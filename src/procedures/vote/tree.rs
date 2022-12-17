use std::sync::{Arc, Weak};

use parking_lot::RwLock;

use crate::{
    entry::Entry,
    log::log_debug,
    node::Node,
    procedures::vote::{node::Vote as NodeVote, VoteResult},
};

pub use crate::tree::TreeExt;

pub trait Vote {
    fn vote(&self, name: &str) -> VoteResult
    where
        Self: TreeExt,
    {
        log_debug("------------------------");
        let mut counter: usize = 0;
        // We need to free the Arc lock before starting the second loop
        let mut found: Option<Entry> = None;

        let mut next: Weak<RwLock<Node>> = Arc::downgrade(self.get_node());

        log_debug("Starting loop post_entry");
        loop {
            if found.is_some() {
                break;
            }

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
                VoteResult::Success(entry, lock) => {
                    log_debug("post_entry Success");
                    match lock {
                        Some(lock) => {
                            // Loop to previous nodes, updating top entries
                            // repeat until it finds the first node or
                            // until it reaches a node where it can't include to the top list
                            log_debug("post_entry Starting update_top_backward");

                            found = Some(entry.clone());
                            lock.clone()
                        }
                        None => {
                            log_debug("post_entry returning Success with entry {entry}");
                            return VoteResult::Success(entry, None);
                        }
                    }
                }
            };
        }

        Self::update_vote_backward(next, &found.unwrap())
    }

    fn update_vote_backward(lock: Weak<RwLock<Node>>, entry: &Entry) -> VoteResult
    where
        Self: TreeExt,
    {
        log_debug("------------------------");
        log_debug("function update_top_backward : calling backward_update_top");
        let mut lock: Weak<RwLock<Node>> = match lock
            .upgrade()
            .expect("Tried to upgrade Lock but got None .")
            .write()
            .backward_update_vote(entry)
        {
            Some(value) => {
                log_debug("Some");
                value
            }
            None => {
                log_debug("None");
                return VoteResult::Success(entry.clone(), None);
            }
        };

        log_debug("Starting Loop");
        loop {
            log_debug("------------------------");
            log_debug("Getting Lock");
            lock = match lock
                .upgrade()
                .expect("Tried to upgrade lock (backwards) but got None .")
                .write()
                .backward_update_vote(entry)
            {
                Some(value) => {
                    log_debug("Returning backward reference .");
                    value
                }
                None => {
                    log_debug("Last backward node found .");
                    return VoteResult::Success(entry.clone(), None);
                }
            }
        }
    }
}
