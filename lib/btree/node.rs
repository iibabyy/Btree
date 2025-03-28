mod key;
mod iterator;


use std::sync::Arc;
use iterator::{ChildIterator, KeysIterator};
use key::Key;

static MAXIMUM_KEYS: usize = 4;

//----------[ NODE ]----------//

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum NodeType {
	Internal,
	Leaf
}

/// This struct contains the indexes with the Nodes pointed by those indexes, and the last node
///
/// The maximum number of keys a Node can contains is defined by the MAXIMUM_KEYS constant variable
/// 
/// # Representation
/// 	Node [
/// 		3->[1, 2],	// Index 3 pointing on a node
/// 		5->[3, 4],	// Index 5 pointing on a node
/// 		[6, 7]	// Last node
/// 	]

#[derive()]
pub struct Node<K, V, const M: usize>
where
	K: Ord
{
	keys: Vec<Key<K, V, M>>,

	type_: NodeType,

	last_node: Option<Box<Node<K, V, M>>>,

	parent: Key<K, V, M>,
}

impl<K, V, const M: usize> Node<K, V, M>
where
	K: Ord
{
	pub fn merge_childs(&mut self, index: usize) -> Option<&Node<K, V, M>> {
		// normally checking if self is a Leaf node
		// but if everithing is done correctly, no need to

		{
			// Remove the key that was linking the node we will remove
			let mut key = self.remove_key(index)?;

			// node we will remove
			let left_node = key.pointed_node_mut()?;

			let key_count = self.keys.len();
			let right_key_index = index + 1;

			// the node where (the key linking the removed node + all the removed node's keys) will be sent
			let right_node = match self.get_mut(right_key_index) {
				Some(key) => key.pointed_node_mut()?,
				None if right_key_index == key_count && key_count > 0  => self.last_node.as_mut().unwrap().as_mut(),
				None => return None,
			};

			// sending the removed node's keys
			while let Some(key) = left_node.pop_last() {
				right_node.insert(key);
			}

			// sending the key linking the removed node
			right_node.insert(key);

			Some(())
		}?;

		match index == self.keys.len() { // check if merged node is the last of the node  
			true => {
				Some(self.last_node.as_ref().unwrap().as_ref())
			},
			false => {
				self.get(index).unwrap().pointed_node()
			}
		}

	}

	pub fn split_into_parent<'a>(&mut self, new_parent: &'a mut Key<K, V, M>) -> &'a mut Key<K, V, M> {
		let mut new_node = Node::new(self.type_, new_parent);

		let keys_to_move = self.keys.len() / 2;

		for _ in 0..keys_to_move {
			let removed_key = self.pop_last().unwrap();

			new_node.insert(removed_key);
		}

		new_parent.set_pointed_node(new_node);

		new_parent
	}

	pub fn split(&mut self) -> Key<K, V, M> {

	}
}

impl<K, V, const M: usize> Node<K, V, M>
where
	K: Ord
{
	pub fn new(type_: NodeType, parent: Key<K, V, M>) -> Self {
		Self {
			type_,
			keys: Vec::with_capacity(MAXIMUM_KEYS + 1),
			last_node: None,
			parent
		}
	}

	pub fn insert(&mut self, key: Key<K, V, M>) -> &Key<K, V, M> {
		let search = self.keys.binary_search(&key);

		match search {
			// If Leaf Node
			Err(index) | Ok(index) if self.type_ == NodeType::Leaf => {
				// Leaf Node, inserting the key in this node				
				self.keys.insert(index, key);
				&self.keys[index]
			},

			// If not Leaf Node
			Err(index) | Ok(index) => {
				let child = self.keys[index].pointed_node_mut().unwrap();
				child.insert(key)
			},
		}
	}

	pub fn find(&self, key: Arc<Key<K, V, M>>) -> Option<&Key<K, V, M>> {
		let search = self.keys.binary_search(&key);

		match search {
			Err(index) => {
				// Key not found
	
				if let Some(next_child) = self.keys[index].pointed_node() {
					return next_child.find(key);
				} else {
					// Leaf node (no child)
					return None;
				}
			},
			Ok(index) => {
				// Key found

				return Some(&self.keys[index])
			}
		};
	}

	pub fn get(&self, index: usize) -> Option<&Key<K, V, M>> {
		self.keys.get(index)
	}

	pub fn get_mut(&mut self, index: usize) -> Option<&mut Key<K, V, M>> {
		self.keys.get_mut(index)
	}

	pub fn remove_key(&mut self, index: usize) -> Option<Key<K, V, M>> {

		if index >= self.keys.len() {
			return None
		}

		let key = self.keys.remove(index);

		return Some(key);
	}

	pub fn pop_first(&mut self) -> Option<Key<K, V, M>> {
		self.remove_key(0)
	}

	pub fn pop_middle(&mut self) -> Option<Key<K, V, M>> {
		let index = self.keys.len() / 2;
		
		self.remove_key(index)
	}

	pub fn pop_last(&mut self) -> Option<Key<K, V, M>> {
		self.keys.pop()
	}

	pub fn insert_keys(&mut self, keys: Vec<Key<K, V, M>>) {
		for key in keys {
			self.insert(key);
		}
	}

	pub fn overflow(&self) -> bool {
		self.keys.len() > MAXIMUM_KEYS
	}

	pub fn is_full(&self) -> bool {
		self.keys.len() >= MAXIMUM_KEYS
	}
	pub fn is_empty(&self) -> bool {
		self.keys.len() >= MAXIMUM_KEYS
	}

	pub fn keys_count(&self) -> usize {
		self.keys.len()
	}

	pub fn child_count(&self) -> usize {
		if self.type_() == NodeType::Leaf {
			// leaf node -> no childs
			0
		} else {
			// number of keys + 1 (for the last node)
			self.keys.len() + 1
		}
	}

	pub fn type_(&self) -> NodeType {
		self.type_
	}

	pub fn key_iter<'a>(&self) -> KeysIterator<'_, K, V, M> {
		KeysIterator::new(self)
	}

	pub fn child_iter<'a>(&self) -> ChildIterator<'_, K, V, M> {
		ChildIterator::new(self)
	}
	
}

//------------------------------//

//----------[ ERRORS ]----------//

#[derive(Debug)]
pub enum NodeError {
	Operation(NodeOperationError),
}

impl From<NodeOperationError> for NodeError {
	fn from(error: NodeOperationError) -> Self {
		Self::Operation(error)
	}
}

impl std::fmt::Display for NodeError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			NodeError::Operation(err) => write!(f, "Operation failed: {err}")
		}
	}
}

impl std::error::Error for NodeError {}

#[derive(Debug)]
pub enum NodeOperationError {
	Full,
	Empty
}

impl std::fmt::Display for NodeOperationError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			NodeOperationError::Full => write!(f, "The Node is full"),
			NodeOperationError::Empty => write!(f, "The Node is empty")
		}
	}
}

impl std::error::Error for NodeOperationError {}

//----------------------//