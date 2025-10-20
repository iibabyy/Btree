use std::collections::VecDeque;

use crate::{key::Key, NodePointer};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum NodeType {
    Internal,
    Leaf,
}

#[derive(Debug)]
pub(crate) struct Node<K, V, const D: usize>
where
    K: Ord,
{
    pub keys: VecDeque<Key<K, V, D>>,
    pub last_node: NodePointer<K, V, D>,
    pub type_: NodeType,

    max_number_of_keys: usize,
    min_number_of_keys: usize,
}

impl<K, V, const D: usize> Node<K, V, D>
where
    K: Ord,
{
    pub(crate) fn new(
        mut keys: VecDeque<Key<K, V, D>>,
        last_node: NodePointer<K, V, D>,
        type_: NodeType,
    ) -> Self {
        if keys.capacity() < D {
            let temp_vec = keys;
            keys = VecDeque::with_capacity(D * 2);
            keys.extend(temp_vec);
        }

        Self {
            keys,
            last_node,
            type_,

            min_number_of_keys: D - 1,
            max_number_of_keys: D * 2 - 1,
        }
    }

    pub(crate) fn leaf(keys: VecDeque<Key<K, V, D>>) -> Self {
        Self::new(keys, None, NodeType::Leaf)
    }

    pub(crate) fn internal(keys: VecDeque<Key<K, V, D>>, last_node: NodePointer<K, V, D>) -> Self {
        Self::new(keys, last_node, NodeType::Internal)
    }

    pub fn default(type_: NodeType) -> Self {
        Self {
            keys: VecDeque::with_capacity(D * 2),
            last_node: None,
            type_,

            min_number_of_keys: D - 1,
            max_number_of_keys: D * 2 - 1,
        }
    }

    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }

    pub fn into_ptr(self) -> Option<Box<Self>> {
        Some(self.boxed())
    }

    pub fn is_leaf(&self) -> bool {
        self.type_ == NodeType::Leaf
    }

    pub fn overflow(&self) -> bool {
        self.keys.len() > self.max_number_of_keys
    }

    pub fn underflow(&self) -> bool {
        self.keys.len() < self.min_number_of_keys
    }

    pub fn len(&self) -> usize {
        self.keys.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<K, V, const D: usize> Node<K, V, D>
where
    K: Ord,
{
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let key = Key::new(key, value);

        let (removed_value, overflow_node_key_index) = self.insert_inner(key);

        if overflow_node_key_index.is_some() {
            self.split_root();
        }

        removed_value
    }

    pub fn insert_inner(&mut self, key: Key<K, V, D>) -> (Option<V>, Option<usize>) {
        let (removed_value, key_index) = if self.is_leaf() {
            self.insert_inner_leaf(key)
        } else {
            let (removed_value, overflow_node_key_index, key_index) = self.insert_inner_internal(key);

            if let Some(overflow_index) = overflow_node_key_index {
                self.split_node_pointed_by_key(overflow_index);
            }

            (removed_value, key_index)
        };

        (
            removed_value,
            self.overflow().then_some(key_index)
        )
    }

    fn insert_inner_internal(&mut self, key: Key<K, V, D>) -> (Option<V>, Option<usize>, usize) {
        let index = self.index_for(&key);

        let (removed_value, overflow_node_key_index) = if index < self.len() {
            let target_key = &mut self.keys[index];

            if key == *target_key {
                (
                    Some(std::mem::replace(&mut target_key.value, key.value)),
                    None,
                )
            } else {   
                target_key
                .pointed_node
                .as_mut()
                .expect("Internal node key isn't pointing on a Node")
                .as_mut()
                .insert_inner(key)
            }
        } else {
            self.last_node
                .as_mut()
                .expect("Internal node key isn't pointing on a Node")
                .as_mut()
                .insert_inner(key)
        };

        (
            removed_value,
            overflow_node_key_index,
            index
        )
    }

    fn insert_inner_leaf(&mut self, key: Key<K, V, D>) -> (Option<V>, usize) {
        let index_returned;

        (
            match self.keys.binary_search(&key) {
                Ok(index) => {
                    index_returned = index;
                    Some(self.keys[index].replace_value(key))
                },
                Err(index) => {
                    index_returned = index;

                    self.keys.insert(index, key);
                    None
                }
            },
            index_returned,
        )
    }

    pub fn remove(&mut self, _key: K) -> Option<V> {
        todo!()
    }

    pub fn slide_one_key_to_left_sibling(
        &mut self,
        left_sibling: &mut Self,
        parent_key: &mut Key<K, V, D>,
    ) -> Result<(), ()> {
        if let Some(mut left_key) = self.keys.pop_front() {
            let prev_left_key_pointed_node =
                std::mem::replace(&mut left_key.pointed_node, parent_key.pointed_node.take());
            parent_key.pointed_node = left_sibling.last_node.take();
            left_sibling.last_node = prev_left_key_pointed_node;

            let prev_parent_key = parent_key.replace(left_key);
            left_sibling.keys.push_back(prev_parent_key);

            Ok(())
        } else {
            Err(())
        }
    }

    pub fn slide_one_key_to_right_sibling(
        &mut self,
        right_sibling: &mut Self,
        parent_key: &mut Key<K, V, D>,
    ) -> Result<(), ()> {
        if let Some(mut right_key) = self.keys.pop_back() {
            let prev_right_key_pointed_node =
                std::mem::replace(&mut right_key.pointed_node, parent_key.pointed_node.take());
            parent_key.pointed_node = self.last_node.take();
            self.last_node = prev_right_key_pointed_node;

            let prev_parent_key = parent_key.replace(right_key);
            right_sibling.keys.push_front(prev_parent_key);

            Ok(())
        } else {
            Err(())
        }
    }

    fn split_node_pointed_by_key(&mut self, index: usize) {
        assert!(index <= self.len(), "index out of bounds");

        let pointed_node = if index == self.len() {
            self.last_node
                .as_mut()
                .expect("last node is empty")
                .as_mut()
        } else {
            self.keys[index]
                .pointed_node
                .as_mut()
                .expect("pointed Node is empty")
                .as_mut()
        };

        let (left_part, mut middle_key, right_part) = pointed_node.split_inner();

        *pointed_node = right_part;
        middle_key.pointed_node = left_part.into_ptr();
        self.keys.insert(index, middle_key);
    }

    fn split_root(&mut self) {
        let (left_part, mut middle_key, right_part) = self.split_inner();
        self.last_node = Some(right_part.boxed());
        middle_key.pointed_node = left_part.into_ptr();
        self.init_keys(middle_key);
    }

    fn split_inner(&mut self) -> (Node<K, V, D>, Key<K, V, D>, Node<K, V, D>) {
        let keys = std::mem::take(&mut self.keys);
        let last_node = Option::take(&mut self.last_node);
        let keys_type = last_node.as_ref().unwrap().type_; // all the keys are the same type

        /*--- split left/middle/right keys ---*/

        let middle = self.len() / 2;
        let mut iter = keys.into_iter();
        let left_keys = iter.by_ref().take(middle).collect();
        let mut middle_key = iter.next().unwrap();
        let right_keys = iter.by_ref().take(middle).collect();

        let middle_key_pointed_node = Option::take(&mut middle_key.pointed_node);

        let left_node = Node::new(left_keys, middle_key_pointed_node, keys_type);
        let right_node = Node::new(right_keys, last_node, keys_type);

        (left_node, middle_key, right_node)
    }
}

impl<K, V, const D: usize> Node<K, V, D>
where
    K: Ord,
{
    fn index_for(&self, key: &Key<K, V, D>) -> usize {
        self.keys.binary_search(key).unwrap_or_else(|err| err)
    }

    fn new_keys() -> VecDeque<Key<K, V, D>> {
        VecDeque::with_capacity(D * 2)
    }

    fn init_keys(&mut self, key: Key<K, V, D>) {
        self.keys = Self::new_keys();

        self.keys.push_front(key);
    }
    // pub fn search_node_for(&mut self, key: &Key<K, V, D>) -> Option<&mut Node<K, V, D>> {
    //     if self.is_leaf() {
    //         return Some(self);
    //     }

    //     if let Err(index) = self.keys.binary_search(&key) {
    //         if index == self.keys.len() {
    //             self.last_node
    //                 .as_mut()
    //                 .map(|ptr| ptr.as_mut().search_node_for(key))
    //                 .unwrap_or(None)
    //         } else {
    //             self.keys[index].pointed_node
    //                 .as_mut()
    //                 .map(|ptr| ptr.as_mut().search_node_for(key))
    //                 .unwrap_or(None)
    //         }
    //     } else {
    //         Some(self)
    //     }
    // }
}
