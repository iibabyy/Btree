use crate::{
    key::Key, node::Node
};

pub struct Btree<K, V, const S: usize = 4>
where
    K: Ord,
{
    root: Node<K, V, S>,
    size: usize,
}

impl<K, V, const S: usize> Btree<K, V, S>
where
    K: Ord,
{
    pub fn new() -> Self {
        Self {
            root: Node::default(),
            size: 0,
        }
    }

    pub fn empty(&self) -> bool {
        self.size == 0
    }

    pub fn len(&self) -> usize {
        self.size
    }
}

impl<K, V, const S: usize> Btree<K, V, S>
where
    K: Ord,
{
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
		let key = Key::new(key, value);
		self.root.insert(key)
    }

	pub fn remove(&mut self, key: K) -> Option<V> {
		self.root.remove(key)
	}
}

impl<K, V, const S: usize> Btree<K, V, S>
where
    K: Ord,
{

}