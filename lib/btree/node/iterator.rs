use super::*;

pub struct KeysIterator<'a, K, V, const M: usize>
where
	K: Ord
{
	node: &'a Node<K, V, M>,
	index: usize
}

impl<'a, K, V, const M: usize> Iterator for KeysIterator<'a, K, V, M>
where
	K: Ord
{
	type Item = &'a Key<K, V, M>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.index < self.node.keys_count() {
			let result = Some(&self.node.keys[self.index]);
			self.index += 1;
			result
		} else {
			None
		}
	}
}

impl<'a, K, V, const M: usize> KeysIterator<'a, K, V, M>
where
	K: Ord
{
	pub(super) fn new(node: &'a Node<K, V, M>) -> Self {
		Self {
			node,
			index: 0
		}
	}
}


pub struct ChildIterator<'a, K, V, const M: usize>
where
	K: Ord
{
	node: &'a Node<K, V, M>,
	index: usize
}

impl<'a, K, V, const M: usize> Iterator for ChildIterator<'a, K, V, M>
where
	K: Ord
{
	type Item = &'a Node<K, V, M>;

	fn next(&mut self) -> Option<Self::Item> {
		let key_count = self.node.keys_count();
		let keys = &self.node.keys;

		match self.index {
			index if index < key_count => {
				let res = keys[self.index].pointed_node();
				self.index += 1;
				res
			},

			index if index == key_count && key_count > 0 => {
				let res = Some(self.node.last_node.as_ref().unwrap().as_ref());
				self.index += 1;
				res
			},

			_ => {
				None
			}
		}
	}
}

impl<'a, K, V, const M: usize> ChildIterator<'a, K, V, M>
where
	K: Ord
{
	pub(super) fn new(node: &'a Node<K, V, M>) -> Self {
		Self {
			node,
			index: 0
		}
	}
}
