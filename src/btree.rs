
use crate::{key::Key, node::{Node, NodeType}, utils::error::BtreeError};

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
            root: Node::new(NodeType::Leaf),
            size: 0,
        })
    }

    pub fn empty(&self) -> bool {
        self.size == 0
    }

    pub fn len(&self) -> usize {
        self.size
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
            self.split_root();
        }

        todo!()
    }

    pub fn split_root(&mut self) {
        let keys = std::mem::replace(&mut self.root.keys, vec![]);
        let last_node = std::mem::replace(&mut self.root.last_node, None);
        let keys_type = last_node.as_ref().unwrap().type_;

        let mid = self.root.len() / 2;

        let mut iter = keys.into_iter();
        let left_keys = iter.by_ref().take(mid).collect();
        let mut middle_key = iter.next().unwrap();
        let right_keys = iter.by_ref().take(mid).collect();

        let middle_key_pointed_node = std::mem::replace(&mut middle_key.pointed_node, None);

        let left_node = Node::with(left_keys, middle_key_pointed_node, keys_type);
        let right_node = Node::with(right_keys, last_node, keys_type);

        middle_key.pointed_node = Some(left_node.boxed());
        let new_root_last_node = right_node.boxed();

        let new_root = Node::with(vec![middle_key], Some(new_root_last_node), NodeType::Internal);

        self.root = new_root;
    }

    pub fn remove(&mut self, key: K) -> Option<V> {
        self.root.remove(key)
    }
}

impl<K, V, const D: usize> Btree<K, V, D> where K: Ord {}
