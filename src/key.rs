use crate::{node::Node};

#[derive(Debug)]
pub struct Key<K, V, const S: usize>
where
    K: Ord,
{
    pub(crate) key: K,
    pub(crate) value: V,
    pub(crate) pointed_node: Option<Box<Node<K, V, S>>>,
}

impl<K, V, const S: usize> Eq for Key<K, V, S> where K: Ord {}
impl<K, V, const S: usize> PartialEq for Key<K, V, S>
where
    K: Ord,
{
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl<K, V, const S: usize> Ord for Key<K, V, S>
where
    K: Ord,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.key.cmp(&other.key)
    }
}

impl<K, V, const S: usize> PartialOrd for Key<K, V, S>
where
    K: Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.key.cmp(&other.key))
    }
}

impl<K, V, const S: usize> Key<K, V, S>
where
    K: Ord,
{
    pub fn new(key: K, value: V) -> Self {
        Self {
            key,
            value,
            pointed_node: None,
        }
    }
}
