use crate::{key::Key, KeyVector, NodePointer};

#[derive(Debug, Copy, Clone)]
pub enum NodeType {
    Internal,
    Leaf,
}

#[derive(Debug, Clone)]
pub struct Node<K, V, const S: usize>
where
	K: Ord
{
    pub(crate) keys: KeyVector<K, V, S>,
    pub(crate) last_node: NodePointer<K, V, S>,
    pub(crate) type_: NodeType,
}

impl<K, V, const S: usize> Default for Node<K, V, S>
where
	K: Ord
{
	fn default() -> Self {
		Self {
			keys: Vec::with_capacity(S),
			last_node: None,
			type_: NodeType::Leaf
		}
	}
}

impl<K, V, const S: usize> Node<K, V, S>
where
	K: Ord
{
    pub(crate) fn new(keys: KeyVector<K, V, S>, last_node: NodePointer<K, V, S>, type_: NodeType) -> Self {
        Self {
            keys,
            last_node,
            type_,
        }
    }

	pub(crate) fn boxed(self) -> Box<Self> {
		Box::new(self)
	}
}

impl<K, V, const S: usize> Node<K, V, S>
where
	K: Ord
{
    pub(crate) fn insert(&mut self, key: Key<K, V, S>) -> Option<V> {
		todo!()
    }

	pub(crate) fn remove(&mut self, key: K) -> Option<V> {
		todo!()
    }
}
