use parking_lot::RwLock;

use crate::{
    entry::Entry,
    env::SuggestionNumber,
    log::log_debug,
    node::NodeExt,
    procedures::get::{GetPrefixError, SearchResult},
    tree::Counter,
};

use std::{collections::HashMap, sync::Arc};

/// Contains all Node logic for GET entry endpoint.
pub trait Get {
    /// Check if current Node is the valid prefix.
    ///
    /// If not valid, return next Node to check.
    ///
    /// If valid, check all children for the top recommendations.
    fn get_top_entries(
        &self,
        prefix: &str,
        counter: &mut Counter,
    ) -> Result<SearchResult, GetPrefixError>
    where
        Self: NodeExt,
    {
        // If this is the node for the given prefix
        let is_correct_node: bool = prefix.len() == self.get_prefix().len();

        log_debug(&format!(
            "\nFunction crate::procedures::get::node::Get::get_top_entries ... Prefix: {}, Times: {}.\n",
            self.get_prefix(),
            self.get_times(),
        ));

        if is_correct_node {
            log_debug("\nIs Correct Node\n");
            log_debug(&format!(
                "Returning result for prefix {} .",
                self.get_prefix()
            ));

            let mut top: Vec<Entry> = Vec::with_capacity(self.get_suggestions());
            let mut result = Vec::with_capacity(self.get_suggestions());

            let mut suggestion_number = self.get_suggestions();

            if self.is_name() {
                let entry = self.get_entry().clone();
                log_debug(&format!("Including first entry: {entry}"));
                result.push(entry);
                suggestion_number -= 1;
            }
            
            self.collect_top_first(&mut top, &suggestion_number);

            result.append(&mut top);

            let entries: usize = result.len();
            log_debug(&format!("Returning {entries} entries."));

            return Ok(SearchResult::Success(result));
        }

        *counter += 1;

        let character: &str = &prefix[(*counter - 1)..*counter];

        match self.next_child(character) {
            None => Err(GetPrefixError::NotFound(prefix.into())),
            Some(value) => Ok(SearchResult::Next(value)),
        }
    }

    // The first entry is ignored, because it is already included as the first recommendation.

    /// Recursively checks all children of current Node for recommendations.
    fn collect_top_first(&self, top: &mut Vec<Entry>, suggestion_number: &SuggestionNumber)
    where
        Self: NodeExt,
    {
        let children: &HashMap<String, Arc<RwLock<Self>>> = self.get_children();

        for (_, child) in children.iter() {
            child.read().collect_top(top, suggestion_number);
        }
    }

    fn collect_top(&self, top: &mut Vec<Entry>, suggestion_number: &SuggestionNumber)
    where
        Self: NodeExt,
    {
        let children: &HashMap<String, Arc<RwLock<Self>>> = self.get_children();

        for (_, child) in children.iter() {
            child.read().collect_top(top, suggestion_number);
        }

        if self.is_name() {
            // If list is not full, include the entry.
            if top.len() < *suggestion_number {
                let entry: Entry = self.get_entry().clone();

                top.push(entry);
                top.sort_by(|a, b| b.cmp(a));

                return;
            }

            let last: &Entry = top
                .last()
                .expect("Unexpected Behavior when retrieving last entry.");

            // If the last (lowest) entry is lower than current, include the current.
            if *last < *self.get_entry() {
                let entry: Entry = self.get_entry().clone();

                // Replace the last entry with current to avoid memory allocation.
                *top.last_mut()
                    .expect("Unexpected Behavior when retrieving last mut entry.") = entry;

                // sorting in reverse so the first entries are the highest.
                top.sort_by(|a, b| b.cmp(a));
            }
        }
    }
}
