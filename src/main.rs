
struct MinHeap<T> {
	data:  Vec::<Box<T>>,
}


impl<T: std::cmp::PartialOrd> MinHeap<T> {

	pub fn new()  -> MinHeap<T> {
		MinHeap { data : Vec::<Box<T>>::new() }
	}

	pub fn insert(&mut self, item: Box::<T> ) {
		self.data.push(item);	
	}

	fn get_parent_index(index: usize) -> Option<usize> {
		if index == 0 {
			None
		} 
		else {
			Some(index-1 / 2)
		}
	}

	fn get_left_child_index(index: usize) -> Option<usize> {
			Some((index-1) * 2)
	}

	fn is_parent(index : usize) -> bool {
		index > 1
	}

	fn less_than_parent(&mut self, current_index: usize) -> bool {
		let parent_index = MinHeap::<T>::get_parent_index(current_index).unwrap();
		// is current value < parent value
		*self.data[current_index] < *self.data[parent_index]
	}

	fn swap_with_parent(&mut self, index: usize) {
		let parent_index = MinHeap::<T>::get_parent_index(index).unwrap();
		let saved =  self.data[parent_index];
		self.data[parent_index] = self.data[index];
		self.data[index] = saved;
	}

	fn heapifyUp(&mut self, index : usize) {
			while MinHeap::<T>::is_parent(index) && self.less_than_parent(index) {
				self.swap_with_parent(index);
			}
	}


}



fn main() {
    println!("Hello, world!");
	let mut v = MinHeap::<u32>::new();
	v.insert(Box::new(1));
	v.insert(Box::new(2));
}
