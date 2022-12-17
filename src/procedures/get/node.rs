use crate::{
    entry::Entry,
    log::log_debug,
    node::NodeExt,
    procedures::get::{GetPrefixError, SearchResult},
    tree::Counter,
};

// Trait Get to be implemented in Node.
pub trait Get {
    fn get_top_entries(&self, prefix: &str, counter: &mut Counter) -> Result<SearchResult, GetPrefixError>
    where
        Self: NodeExt,
    {
        // If this is the node for the given prefix
        let is_correct_node: bool = prefix.len() == self.get_prefix_len();

        log_debug(&format!(
            "\nFunction crate::procedures::get::node::Get::get_top ... Prefix: {}, Times: {}.\n",
            self.get_prefix(),
            self.get_times(),
        ));

        if is_correct_node {
            log_debug("\nIs Correct Node\n");
            log_debug(&format!(
                "Returning result for prefix {} .",
                self.get_prefix()
            ));
            log_debug(&format!("Returning {} entries", self.get_top_len()));
            let mut result = Vec::with_capacity(self.get_suggestions());

            let limit: usize = self.limit();

            let top: &Vec<Entry> = self.get_top();

            // If this node is a name, include it as the first option
            if self.is_name() {
                let name = self.get_prefix();
                let times = self.get_times();
                // let first_option: Entry = Entry {
                //     name: self.get_prefix(),
                //     times: self.get_times(),
                // };
                let first_option: Entry = Entry::new(name, times);
                result.push(first_option);

                // for i in 0..limit {
                //     if top[i] == result[0] {
                //         // if top.len() > limit {
                //         //     limit = self.limit() + 1;
                //         // }

                //         continue;
                //     }

                //     result.push(top[i].clone());
                // }

                for i in top.iter().take(limit) {
                    if *i == result[0]{
                        continue;
                    }

                    result.push(i.clone());
                }

                return Ok(SearchResult::Success(result));
            }

            // for i in 0..limit {
            //     result.push(top[i].clone());
            // }

            for i in top.iter(){
                result.push(i.clone());
            }

            // top.iter().map(|i| result.push(i.clone()));

            return Ok(SearchResult::Success(result));
        }

        *counter += 1;

        let character: &str = &prefix[(*counter - 1)..*counter];

        match self.next_child(character) {
            None => Err(GetPrefixError::NotFound(prefix.into())),
            Some(value) => Ok(SearchResult::Next(value)),
        }
    }
}
