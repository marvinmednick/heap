//use std::cell::Cell;

struct MinHeap<T> {
	data:  Vec::<Box<T>>,
}


impl<T: std::cmp::PartialOrd+std::fmt::Debug> MinHeap<T> {

	pub fn new()  -> MinHeap<T> {
		MinHeap { data : Vec::<Box<T>>::new() }
	}

	pub fn set(&mut self, vec: Vec::<Box<T>>) {
        self.data = vec;
		println!("After set {:?}",self.data);
	}

	pub fn insert(&mut self, item: T ) {
    //    println!("Inserting {:?} ****** ",item);
		let entry = Box::<T>::new(item);
		self.data.push(entry);	
		self.heapify_up(self.data.len()-1);
		println!("After insert {:?}",self.data);
	}

	pub fn update(&mut self, index: usize, new_value: T) {
		if self.valid_index(index) {
			if new_value < *self.data[index] {
					let new_entry = Box::<T>::new(new_value); 
					self.data[index] = new_entry;
					self.heapify_up(index);
			}
			else if new_value > *self.data[index] {
					let new_entry = Box::<T>::new(new_value); 
					self.data[index] = new_entry;
					self.heapify_down(index);
			} 
			// must be equal, so no heap adjust required
			else {
					let new_entry = Box::<T>::new(new_value); 
					self.data[index] = new_entry;
			}

		}
		println!("After update {:?}",self.data);
		
	}

    pub fn validate_heap(&self) -> bool {

        for x in (0..self.data.len()) {
            let left = self.get_left_child_index(x);
            let right = self.get_right_child_index(x);
            let left_valid = left.is_none() || *self.data[left.unwrap()] >= *self.data[x];
            let right_valid = right.is_none() || *self.data[right.unwrap()] >= *self.data[x];
           // println!("Item {} -> left: {:?} right:  {:?} valid: {} {}",x,left,right,left_valid,left_valid);
            if !left_valid || !right_valid {
                println!("INVALID heap");
                return false;
            }
        }
        println!("Valid heap");
        return true;
    }

	pub fn get_min(&mut self) -> T {
		let retval = *self.data.swap_remove(0);
		self.heapify_down(0);
		println!("After get_min {:?}",self.data);
		retval
	}

	fn get_parent_index(&self, index: usize) -> Option<usize> {
		if index == 0 {
			None
		} 
		else {
			Some( (index-1)/2)
		}
	}

	fn get_left_child_index(&self, index: usize) -> Option<usize> {
			let child_index = index*2+1;
			if child_index < self.data.len() {
				Some(child_index)
			}
			else {
				None
			}
	}

	fn get_right_child_index(&self, index: usize) -> Option<usize> {
			let child_index = index*2+2;
			if child_index < self.data.len() {
				Some(child_index)
			}
			else {
				None
			}
	}

	fn valid_index(&self, index: usize) -> bool {
		
		if index < self.data.len() {
			true
		}
		else {
			false
		}
	}


	fn has_parent(&self,index : usize) -> bool {
		index > 0
	}

	fn less_than(&self, a: usize, b: usize) -> bool {
		if self.valid_index(a) && self.valid_index(b) {
			let return_value =	*self.data[a] < *self.data[b];
			return_value
		}
		else {
			println!("Comparing {} {} - {}",a,b,false);
			false
		}
	}

	fn less_than_parent(&mut self, current_index: usize) -> bool {
        let retval : bool;
		if self.has_parent(current_index) {
			let parent_index = self.get_parent_index(current_index).unwrap();
			// is current value < parent value
			retval = *self.data[current_index] < *self.data[parent_index];
		}
		else {
			retval = false;
		}
       // println!("idx: {} Less than parent {}", current_index, retval);
        retval
	}

	fn get_smallest_child_index(&self, index: usize) -> Option<usize> {

		let left_index = self.get_left_child_index(index);
		let right_index = self.get_right_child_index(index);

		println!("index {} Left and righ child indexes {:?}, {:?}",index,left_index,right_index);
		if left_index.is_none() {
			None
		}
		else if right_index.is_none() {
			left_index
		}
		else {
			if *self.data[left_index.unwrap()] <= *self.data[right_index.unwrap()] {
				left_index
			}
			else {
				right_index
			}
		}

	}

	fn swap_with_parent(&mut self, index: usize) {
		let parent_index = self.get_parent_index(index).unwrap();
        self.data.swap(index,parent_index);
	}

	fn heapify_up(&mut self, index : usize) {
            let mut working_index = index.clone();
			while self.less_than_parent(working_index) {
                let new_working_index = self.get_parent_index(working_index).unwrap();
				self.swap_with_parent(working_index);
                working_index = new_working_index;
			}
	}

	fn heapify_down(&mut self, index : usize) {
		// get a local copy of the current index to modify as we 
		// move nodes down the tree
		let mut cur_index = index.clone();

		// get the smallest child (its index and its value) as tuple (index, T)
		// continue until we don't need to swap or we don't have any more children
		while let Some(smallest_child_index) = self.get_smallest_child_index(cur_index) {
			// compare the smallest child value with the current value
			// if the child is smaller, swap with it and proceed down the tree
			if self.less_than(smallest_child_index,cur_index) {
				self.data.swap(cur_index,smallest_child_index);
				// after the swap, the current nodw will be where the child was
				cur_index = smallest_child_index;
			}
			else {
				// child was NOT small, so we're donel
				break;
			}
		}
	}



}



fn main() {
    println!("Hello, world!");
	let mut v = MinHeap::<u32>::new();
	v.insert(10);
	v.insert(5);
	v.insert(1);
	v.insert(3);
	v.update(2,11);
	v.update(3,2);
	v.update(1,2);


	assert_eq!(v.get_min(),1);
	assert_eq!(v.get_min(),2);
	assert_eq!(v.get_min(),3);
	assert_eq!(v.get_min(),11);

	//#[derive(Debug,PartialOrd,PartialEq)]
	#[derive(Debug)]
	struct Person {
		rank: u32,
		age: u32,
		name: String,
	}

    use std::cmp::Ordering;

    impl PartialOrd for Person {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            self.age.partial_cmp(&other.age)
        }
    }

    impl PartialEq for Person {
        fn eq(&self, other: &Self) -> bool {
            self.age == other.age
        }
    }


	let mut v = MinHeap::<u32>::new();
	v.insert(61);
	v.insert(60);
	v.insert(50);
	v.insert(10);
	v.insert(18);
	v.insert(40);
    v.validate_heap();

	let mut v = MinHeap::<Person>::new();
	v.insert(Person { name: "Marvin".to_string(), age:61,  rank: 1});
	v.insert(Person { name: "Marvin".to_string(), age:60,  rank: 2});
	v.insert(Person { name: "Marcia".to_string(), age:50,  rank: 2});
	v.insert(Person { name: "Jordana".to_string(), age:10,  rank: 3});
	v.insert(Person { name: "Gizmo".to_string(), age:18,  rank:4 });
    v.validate_heap();
	v.get_min();
	v.get_min();
	v.get_min();
	
	let mut v = MinHeap::<u32>::new();
    v.set(vec!(Box::new(1),Box::new(3),Box::new(2)));
    v.validate_heap();
    v.set(vec!(Box::new(3),Box::new(2),Box::new(1)));
    v.validate_heap();
}

