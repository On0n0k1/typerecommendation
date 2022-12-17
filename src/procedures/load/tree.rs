use std::{
    collections::HashMap,
    sync::{Arc, Weak},
};

use parking_lot::RwLock;

use crate::{
    entry::Entry,
    log::log_debug,
    node::Node,
    procedures::load::{node::Load as NodeLoad, LoadError},
    tree::{Counter, TreeExt},
};

fn load_json() -> HashMap<String, u32> {
    // Load the file into a string.
    let input_path = "./names.json";
    let text = std::fs::read_to_string(input_path).unwrap();

    // Parse the json File
    let loaded_values: HashMap<String, u32> = serde_json::from_str(&text).unwrap();

    // Uncomment the line below to see all loaded names
    // println!("Hash Map = {:#?}", loaded_values);

    log_debug(&format!("{} elements found.", loaded_values.len()));

    loaded_values
}

pub trait Load {
    fn include(&self, entry: Entry) -> Result<(), LoadError> where
    Self: TreeExt{
        log_debug(&format!("Loading {} .", entry));

        let mut counter: Counter = 0;

        // let mut traveller: Option<Weak<RwLock<Node>>> =
        //     node.write().load(&entry, &mut counter)?;

        let node: &Arc<RwLock<Node>> = self.get_node();

        let mut traveller: Option<Weak<RwLock<Node>>> = node.write().load(&entry, &mut counter)?;

        loop {
            let next_traveller: Option<Weak<RwLock<Node>>> = match &traveller {
                None => break, // Last node was reached, so end the loop
                Some(value) => {
                    // Weak::upgrade returns an Arc pointer for us to use
                    // If value.upgrade() return None, the reference to the child was removed from memory.
                    //
                    // We do this because an Arc pointer keeps the reference alive, while a Weak reference doesn't.
                    // A child node keeping itself alive would result in a memory leak.
                    match value.upgrade() {
                        None => {
                            return Err(LoadError::ReferenceEmptyDuringLoad(entry, counter))
                        }
                        Some(value) => {
                            // Run the load method to get the next child
                            let next: Option<Weak<RwLock<Node>>> =
                                value.write().load(&entry, &mut counter)?;

                            // Return the next child
                            next
                        }
                    }
                }
            };
            traveller = next_traveller;
        }

        log_debug(&format!("Ran through {} nodes.", counter));
        Ok(())
    }

    fn load(&self) -> Result<(), LoadError>
    where
        Self: TreeExt,
    {
        let entries: HashMap<String, u32> = load_json();

        for (name, times) in entries.iter() {
            let entry = Entry::new(name, *times as u64);

            self.include(entry)?;
        }

        Ok(())
    }
}
