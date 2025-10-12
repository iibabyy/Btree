#![allow(dead_code)]

use crate::node::Node;

pub mod btree;
pub mod key;
pub mod node;
pub mod utils;

type NodePointer<K, V, const D: usize> = Option<Box<Node<K, V, D>>>;

pub fn main() {}
