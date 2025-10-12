#![allow(dead_code)]

use crate::node::Node;

pub mod node;
pub mod key;
pub mod btree;
pub mod utils;

type NodePointer<K, V, const D: usize> = Option<Box<Node<K, V, D>>>;

pub fn main() {

}