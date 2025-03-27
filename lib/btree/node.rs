mod key;


use std::sync::Arc;
use key::Key;

static MAXIMUM_KEYS: usize = 4;

//----------[ NODE ]----------//

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
	keys: Vec<Arc<Key<K, V, M>>>,

	last_node: Option<Arc<Node<K, V, M>>>,
}

impl<K, V, const M: usize> Node<K, V, M>
where
	K: Ord
{
	pub fn new() -> Self {
		Self {
			keys: Vec::with_capacity(MAXIMUM_KEYS + 1),
			last_node: None,
		}
	}

	pub fn insert(&mut self, key: Arc<Key<K, V, M>>) {
		let search = self.keys.binary_search(&key);
		match search {
			Err(index) => {
				// Not found, index is the place were the key can be inserted
	
				self.keys.insert(index, key);
			},
			Ok(index) => {
				// Key already present, the index is the place of the duplicate key
				// TODO: be sure inserting a duplicate value is ok

				self.keys.insert(index, key);
			}
		};
	}

	pub fn insert_keys(&mut self, keys: Vec<Arc<Key<K, V, M>>>) {
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
		if self.is_leaf() {
			// leaf node -> no childs
			0
		} else {
			// number of keys + 1 (for the last node)
			self.keys.len() + 1
		}
	}

	pub fn is_leaf(&self) -> bool {
		self.last_node.is_none()
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