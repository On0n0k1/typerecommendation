use std::sync::Weak;

use parking_lot::RwLock;

use crate::{
    entry::Entry,
    log::log_debug,
    node::{Node, NodeExt},
    procedures::vote::VoteResult,
    tree::Counter,
};

pub trait Vote {
    fn vote(&mut self, name: &str, counter: &mut Counter) -> VoteResult
    where
        Self: NodeExt,
    {
        log_debug("\nFunction crate::procedures::vote::Vote::vote\n");
        let is_last_node: bool = *counter == name.len();

        // The value of Entry.times is unknown.
        // Therefore it will not increase the top rankings until we reach the last node.
        // After reaching the last node, it will start travelling backwards updating all the top recommendations.
        //
        // It is more efficient to make integer comparisons than string comparisons in each top list.
        //
        // Plus by comparing top-to-bottom, there's no need to iterate through every node list.

        if is_last_node {
            if !self.is_name() {
                // The node exists but it is not a name
                return VoteResult::NotFound;
            }

            let times = self.get_times() + 1;
            let entry: Entry = Entry::new(self.get_prefix(), times);
            self.set_times(times);

            let lock = self.get_lock();

            return VoteResult::Success(entry, Some(lock));
        }

        let character: &str = &name[*counter..(*counter + 1)];
        log_debug(&format!(
            "Node for character {character} counter {}",
            *counter
        ));

        match self.next_child(character) {
            None => {
                log_debug("Not found");
                VoteResult::NotFound
            }
            Some(lock) => {
                *counter += 1;
                log_debug("Found");

                VoteResult::Next(lock)
            }
        }
    }

    fn backward_update_vote(&mut self, entry: &Entry) -> Option<Weak<RwLock<Node>>>
    where
        Self: NodeExt,
    {
        log_debug("\nFunction crate::procedures::vote::node::Vote::backwards_update_vote ...\n");
        let lowest_times: u64 = match self.get_top_last() {
            None => unreachable!(),
            Some(value) => *value.get_times(),
        };

        if lowest_times > *entry.get_times() {
            // The lowest entry in top recommendations is higher than the current
            // There's nothing else to do
            return None;
        }

        if self.update_top(entry) {
            return self.get_parent();
        }

        self.include_top(entry);

        self.get_parent()
    }
}
