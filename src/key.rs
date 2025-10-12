use crate::{node::Node, NodePointer};

#[derive(Debug, Clone)]
pub struct Key<K, V, const S: usize>
where
	K: Ord
{
	pub(crate) key: K,
	pub(crate) value: V,
	pub(crate) pointed_node: Option<Box<Node<K, V, S>>>,
}

impl<K, V, const S: usize> Key<K, V, S>
where
	K: Ord
{
	pub fn new(key: K, value: V) -> Self {
		Self {
			key,
			value,
			pointed_node: None
		}
	}
}