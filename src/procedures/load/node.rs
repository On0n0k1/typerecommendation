use std::sync::Weak;

use parking_lot::RwLock;

use crate::{
    entry::Entry,
    node::{Node, NodeExt},
    procedures::load::LoadError,
    tree::Counter,
};

pub trait Load {
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

        self.include_top(entry);

        if is_last_node {
            self.set_times(*entry.get_times());

            Ok(None)
        } else {
            let character: &str = &entry.get_name()[*counter..(*counter + 1)];
            let next = self.next_child_create(character);

            *counter += 1;
            Ok(Some(next))
        }
    }
}
