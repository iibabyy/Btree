#![allow(dead_code)]

use crate::{key::Key, node::Node};

pub mod node;
pub mod key;
pub mod btree;
pub mod utils;

type NodePointer<K, V, const S: usize> = Option<Box<Node<K, V, S>>>;
type KeyVector<K, V, const S: usize> = Vec<Key<K, V, S>>;

pub fn main() {

}