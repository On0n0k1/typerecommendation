use std::sync::{ Arc, Weak };

use parking_lot::RwLock;

use crate::{
    endpoints::rec::prefix::get::Output as GetNamesOutput,
    log::log_debug,
    node::Node,
    procedures::get::{node::Get as NodeGet, GetPrefixError, SearchResult},
    tree::TreeExt,
};

pub trait Get {
    fn get_top(&self, prefix: &str) -> Result<GetNamesOutput, GetPrefixError>
    where
        Self: TreeExt,
    {
        log_debug("------------------------");
        let mut counter = 0;
        let first_node: Weak<RwLock<Node>> = Arc::downgrade(self.get_node());
        
        let mut traveller: Weak<RwLock<Node>> = first_node;

        // Repeat until the last node
        loop {
            log_debug("------------------------");
            traveller = match traveller
                .upgrade()
                .expect("Error in get_top_prefix. Upgrading Arc pointer resulted in a None.")
                .read()
                .get_top_entries(prefix, &mut counter)
            {
                Err(err) => {
                    return Err(err);
                }
                Ok(search_result) => match search_result {
                    SearchResult::Next(pointer) => pointer,
                    SearchResult::Success(value) => return Ok(SearchResult::Success(value).into()),
                },
            };
        }
    }
}
