use crate::{key::Key, NodePointer};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum NodeType {
    Internal,
    Leaf,
}

impl NodeType {
    pub(crate) fn new(&self) {}
}

#[derive(Debug)]
pub(crate) struct Node<K, V, const D: usize>
where
    K: Ord,
{
    pub keys: Vec<Key<K, V, D>>,
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

    pub(crate) fn leaf(
        keys: Vec<Key<K, V, D>>,
    ) -> Self {
        Self {
            keys,
            last_node: None,
            type_: NodeType::Leaf,

            min_number_of_keys: D - 1,
            max_number_of_keys: D * 2 - 1,
        }
    }

    pub(crate) fn internal(
        keys: Vec<Key<K, V, D>>,
        last_node: NodePointer<K, V, D>,
    ) -> Self {
        Self {
            keys,
            last_node,
            type_: NodeType::Internal,

            min_number_of_keys: D - 1,
            max_number_of_keys: D * 2 - 1,
        }
    }

    pub fn default(type_: NodeType) -> Self {
        Self {
            keys: Vec::with_capacity(D),
            last_node: None,
            type_,

            min_number_of_keys: D - 1,
            max_number_of_keys: D * 2 - 1,
        }
    }

    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
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
    pub fn insert(&mut self, _key: Key<K, V, D>) -> Option<V> {
        // match self.keys.binary_search(&key) {
        //     Ok(_) => {}
        //     Err(_) => {}
        // }

        todo!()
    }

    pub fn remove(&mut self, _key: K) -> Option<V> {
        todo!()
    }

    pub fn split(&mut self) {
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


        /*--- construct new node ---*/

        let left_node = Node::new(left_keys, middle_key_pointed_node, keys_type);
        let right_node = Node::new(right_keys, last_node, keys_type);

        middle_key.pointed_node = Some(left_node.boxed());
        let new_last_node = right_node.boxed();
        let mut new_keys = Vec::with_capacity(D);
        new_keys.push(middle_key);

        *self = Node::internal(new_keys, Some(new_last_node));
    }
}

impl<K, V, const D: usize> Node<K, V, D>
where
    K: Ord,
{
    pub fn child_node_for(&mut self, _key: Key<K, V, D>) -> Option<NodePointer<K, V, D>> {
        if self.is_leaf() {
            return None;
        }

        // self.keys.bi

        todo!()
    }
}
