use std::sync::Weak;

use parking_lot::RwLock;

use crate::{
    entry::Entry,
    node::{Node, NodeExt},
    procedures::load::LoadError,
    tree::Counter,
};

/// Contains all Node logic for loading entries into the prefix tree.
pub trait Load {
    /// Checks if current Node is valid for given Entry.
    ///
    /// If not valid, returns next Node to access, creates new Nodes as needed.
    ///
    /// If valid, assign 'entry.times' to this Node.
    ///
    /// # Errors
    ///
    /// If Entry name is empty, returns Err(LoadError).
    fn load(
        &mut self,
        entry: &Entry,
        counter: &mut Counter,
    ) -> Result<Option<Weak<RwLock<Node>>>, LoadError>
    where
        Self: NodeExt,
    {
        if entry.get_name().is_empty() {
            return Err(LoadError::EntryNameIsEmpty(entry.clone(), *counter));
        }

        // If this is the last node (character) of the name
        let is_last_node: bool = *counter == entry.get_name().len();

        if is_last_node {
            *self.get_times_mut() = *entry.get_times();

            Ok(None)
        } else {
            let character: &str = &entry.get_name()[*counter..(*counter + 1)];
            let next = self.next_child_create(character);

            *counter += 1;
            Ok(Some(next))
        }
    }
}
