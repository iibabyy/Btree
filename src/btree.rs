use crate::{
    key::Key,
    node::{Node, NodeType},
    utils::error::BtreeError,
};

const MINIMUM_LOWER_BOUND: usize = 2;
const MAXIMUM_LOWER_BOUND: usize = 2048;

pub struct Btree<K, V, const D: usize = 4>
where
    K: Ord,
{
    root: Node<K, V, D>,
    size: usize,
}

impl<K, V, const D: usize> Btree<K, V, D>
where
    K: Ord,
{
    pub fn new() -> Result<Self, BtreeError> {
        if D < MINIMUM_LOWER_BOUND {
            return Err(BtreeError::LowerBoundTooLow);
        }
        if D > MAXIMUM_LOWER_BOUND {
            return Err(BtreeError::LowerBoundTooLarge);
        }

        Ok(Self {
            root: Node::default(NodeType::Leaf),
            size: 0,
        })
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
}

impl<K, V, const D: usize> Btree<K, V, D>
where
    K: Ord,
{
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let key = Key::new(key, value);

        self.root.insert(key);

        if self.root.overflow() {
            self.root.split();
        }

        todo!()
    }

    pub fn remove(&mut self, key: K) -> Option<V> {
        self.root.remove(key)
    }
}