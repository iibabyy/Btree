use crate::{key::Key, NodePointer};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum NodeType {
    Internal,
    Leaf,
}

#[derive(Debug)]
pub struct Node<K, V, const D: usize>
where
    K: Ord,
{
    pub(crate) keys: Vec<Key<K, V, D>>,
    pub(crate) last_node: NodePointer<K, V, D>,
    pub(crate) type_: NodeType,

    max_number_of_keys: usize,
    min_number_of_keys: usize,
}

impl<K, V, const D: usize> Node<K, V, D>
where
    K: Ord,
{
    pub(crate) fn new(type_: NodeType) -> Self {
        Self {
            keys: Vec::with_capacity(D),
            last_node: None,
            type_,

            min_number_of_keys: D - 1,
            max_number_of_keys: D * 2 - 1,
        }
    }

    pub(crate) fn with(
        keys: Vec<Key<K, V, D>>,
        last_node: NodePointer<K, V, D>,
        type_: NodeType,
    ) -> Self {
        Self {
            keys,
            last_node,
            type_,

            min_number_of_keys: D - 1,
            max_number_of_keys: D * 2 - 1,
        }
    }

    pub(crate) fn boxed(self) -> Box<Self> {
        Box::new(self)
    }

    pub(crate) fn is_leaf(&self) -> bool {
        self.type_ == NodeType::Leaf
    }

    pub(crate) fn overflow(&self) -> bool {
        self.keys.len() > self.max_number_of_keys
    }

    pub(crate) fn underflow(&self) -> bool {
        self.keys.len() < self.min_number_of_keys
    }

    pub const fn len(&self) -> usize {
        self.keys.len()
    }
}

impl<K, V, const D: usize> Node<K, V, D>
where
    K: Ord,
{
    pub(crate) fn insert(&mut self, key: Key<K, V, D>) -> Option<V> {
        match self.keys.binary_search(&key) {
            Ok(_) => {}
            Err(_) => {}
        }

        todo!()
    }

    pub(crate) fn remove(&mut self, key: K) -> Option<V> {
        todo!()
    }
}

impl<K, V, const D: usize> Node<K, V, D>
where
    K: Ord,
{
    pub(crate) fn child_node_for(&mut self, key: Key<K, V, D>) -> Option<NodePointer<K, V, D>> {
        if self.is_leaf() {
            return None;
        }

        // self.keys.bi

        todo!()
    }
}
