use crate::{
    entry::Entry, log::log_debug, node::NodeExt, procedures::vote::VoteResult, tree::Counter,
};

/// Contains all Node logic for incrementing 'times' related to a given name.
pub trait Vote {
    /// Checks if current Node is valid for given Entry.
    ///
    /// If not valid, returns next Node to access, does not create new nodes.
    ///
    /// If valid, increment 'entry.times' on this Node.
    ///
    /// If there are no remaining Nodes to access, returns VoteResult::NotFound.
    fn vote(&mut self, name: &str, counter: &mut Counter) -> VoteResult
    where
        Self: NodeExt,
    {
        log_debug("\nFunction crate::procedures::vote::Vote::vote\n");
        let is_last_node: bool = *counter == name.len();

        if is_last_node {
            if !self.is_name() {
                // The node exists but it is not a name
                return VoteResult::NotFound;
            }

            let times = self.get_times() + 1;
            let entry: Entry = Entry::new(self.get_prefix().into(), times);
            *self.get_times_mut() = times;

            return VoteResult::Success(entry);
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
}
