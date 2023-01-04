mod ext;

use std::sync::Arc;

use parking_lot::RwLock;

use crate::{
    node::Node,
    procedures::{
        get::tree::Get,
        load::{tree::Load, LoadError},
        vote::tree::Vote,
    },
};

pub use crate::tree::ext::TreeExt;

pub type Counter = usize;

/// Prefix Tree for storing values of type Entry on each Node.
///
/// Starts with a Node of name="" and times=0 as the starting point.
///
/// All child Nodes are created as entries are included.
///
/// Can be cloned. All clones will point to the same starting Node.
/// Sharing the same data with each unique thread.
pub struct Tree {
    // The first node
    node: Arc<RwLock<Node>>,
}

// Cloning the tree will create another atomic pointer to the same node.
impl Clone for Tree {
    fn clone(&self) -> Self {
        let node = Arc::clone(&self.node);
        Self { node }
    }
}

impl TreeExt for Tree {
    fn get_node(&self) -> &Arc<RwLock<Node>> {
        &self.node
    }
}

impl Get for Tree {}

impl Vote for Tree {}

impl Load for Tree {}

impl Tree {
    /// Creates an instance of Tree and load entries from '/names.json' .
    pub async fn new(suggestions: usize) -> Result<Self, LoadError> {
        let tree: Tree = Tree::new_empty(suggestions).await;
        tree.load()?;

        Ok(tree)
    }

    /// Creates an empty instance of Tree. Used for testing.
    pub async fn new_empty(suggestions: usize) -> Self {
        let node: Arc<RwLock<Node>> = Node::new(None, "".into(), 0, suggestions);
        Tree { node }
    }
}
