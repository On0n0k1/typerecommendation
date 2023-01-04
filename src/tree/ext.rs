use std::sync::Arc;

use parking_lot::RwLock;

use crate::node::Node;

/// Contains Logic related to the Prefix Tree that is used by all other traits.
pub trait TreeExt {
    /// Returns the atomic pointer for the first Node in the Tree.
    fn get_node(&self) -> &Arc<RwLock<Node>>;
}
