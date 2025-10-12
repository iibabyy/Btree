#![allow(dead_code)]

use crate::node::Node;

pub mod node;
pub mod key;
pub mod btree;
pub mod utils;

type NodePointer<K, V, const S: usize> = Option<Box<Node<K, V, S>>>;

pub fn main() {

}