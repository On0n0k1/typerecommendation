use std::fmt::Display;

use crate::{entry::Entry, tree::Counter};

pub enum LoadError {
    EntryNameIsEmpty(Entry, Counter),
    ReferenceEmptyDuringLoad(Entry, Counter),
}

impl Display for LoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            Self::EntryNameIsEmpty(entry, counter) => write!(f, "Loading JSON Error: Entry has an empty name. Counter: {}, Entry: {} .", counter, entry),
            Self::ReferenceEmptyDuringLoad(entry, counter) => write!(f, "Loading JSON Error: Loaded a node with a non-existent self-reference. Counter: {}, Entry: {} .", counter, entry),
        }
    }
}
