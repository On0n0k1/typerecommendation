use std::sync::Arc;

use parking_lot::RwLock;

use crate::node::Node;

pub trait TreeExt {
    fn get_node<'a>(&'a self) -> &Arc<RwLock<Node>>;
}
