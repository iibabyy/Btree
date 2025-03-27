use super::*;

//----------[ KEY ]----------//

pub struct Key<K, V, const M: usize>
where
	K: Ord
{
	key: K,
	value: V,
	pointed_node: Option<Node<K, V, M>>,
}

impl<K, V, const M: usize> std::cmp::PartialEq for Key<K, V, M>
where
	K: Ord 
{
	fn eq(&self, other: &Self) -> bool {
		self.key == other.key
	}
}

impl<K, V, const M: usize> Eq for Key<K, V, M>
where
	K: Ord
{}

impl<K, V, const M: usize> Ord for Key<K, V, M>
where
	K: Ord
{
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		self.key.cmp(&other.key)
	}
}

impl<K, V, const M: usize> std::cmp::PartialOrd for Key<K, V, M>
where
	K: Ord 
{
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

impl<K, V, const M: usize> Key<K, V, M>
where
	K: Ord 
{
	pub fn new(key: K, value: V, pointed_node: Option<Node<K, V, M>>) -> Self {
		Self {
			key,
			value,
			pointed_node
		}
	}

	fn key(&self) -> &K {
		&self.key
	}

	fn value(&self) -> &V {
		&self.value
	}

	fn pointed_node(&self) -> Option<&Node<K, V, M>> {
		self.pointed_node.as_ref()
	}

}

//----------------------------//
