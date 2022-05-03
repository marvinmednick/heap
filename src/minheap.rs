use std::collections::HashMap;

#[derive(Debug)]
pub struct HeapDataItem<T> {
    id:    u32,
    data: Box<T>
}

pub struct MinHeap<T> {
    heap_contents:  Vec::<HeapDataItem<T>>,
    index_by_id:   HashMap::<u32,usize>,
    next_id: u32,
}


impl<T: std::cmp::PartialOrd+std::fmt::Debug> MinHeap<T> {

    pub fn new()  -> MinHeap<T> {
        MinHeap { 
            heap_contents : Vec::<HeapDataItem<T>>::new(), 
            index_by_id:  HashMap::<u32,usize>::new(),
            next_id:  0
        }

    }


    pub fn set(&mut self, mut vec: Vec::<Box<T>>) {
        let mut new_heapvec = Vec::<HeapDataItem<T>>::new();
        let mut next_id = 0;
        for i in 0..vec.len() {
            let item = vec.remove(0);
            let new_item = HeapDataItem {id: next_id, data: item};
            println!("Adding {:?}",new_item);
            new_heapvec.push(new_item);
            next_id += 1
        }
            
        self.heap_contents = new_heapvec;
        println!("After set Contents {:?}",self.heap_contents);
        println!("After set Index {:?}",self.index_by_id);
    }
    

    pub fn insert(&mut self, item: T ) {
    //    println!("Inserting {:?} ****** ",item);
        let entry = HeapDataItem { id: self.next_id, data: Box::<T>::new(item)};
        // add the entry
        self.heap_contents.push(entry);	
        // update the index by id so there is a mapping from id to its current 
        // locatio withing the heap
        self.index_by_id.insert(self.next_id,self.heap_contents.len()-1);
        // update the next_id for the next insert
        self.next_id += 1;
        // fix up the heap
        self.heapify_up(self.heap_contents.len()-1);
        println!("After insert {:?}",self.heap_contents);
    }

    fn replace(&mut self,index: usize, new_value: T) {
            let new_entry = HeapDataItem { id: self.next_id, data: Box::<T>::new(new_value)}; 
            self.next_id += 1;
            let old_id = self.heap_contents[index].id.clone();
            // remove the entry in the index_by_id map
            self.index_by_id.remove(&old_id);
            // setup the index first, before its moved to the heap
            self.index_by_id.insert(new_entry.id,index.clone());
            self.heap_contents[index] = new_entry;
    }

    pub fn update(&mut self, index: usize, new_value: T) {
        if self.valid_index(index) {

            if new_value < *self.heap_contents[index].data {
                    self.replace(index,new_value);
                    self.heapify_up(index);
            }
            else if new_value > *self.heap_contents[index].data {
                    self.replace(index,new_value);
                    self.heapify_down(index);
            } 
            // values are be equal, so no heap adjust required
            else {
                    self.replace(index,new_value);
            }

        }
        println!("After update {:?}",self.heap_contents);
        
    }

    pub fn validate_heap(&self) -> bool {

        for x in 0..self.heap_contents.len() {
            let left = self.get_left_child_index(x);
            let right = self.get_right_child_index(x);
            let left_valid = left.is_none() || *self.heap_contents[left.unwrap()].data >= *self.heap_contents[x].data;
            let right_valid = right.is_none() || *self.heap_contents[right.unwrap()].data >= *self.heap_contents[x].data;
            println!("Item {} -> left: {:?} right:  {:?} valid: {} {}",x,left,right,left_valid,left_valid);
            if !left_valid {
                    println!("Left invalid")
            }
            if !right_valid {
                    println!("Right invalid")
            }
            if !left_valid || !right_valid {
                println!("INVALID heap");
                return false;
            }
        }
        println!("Valid heap");
        return true;
    }

    pub fn get_min(&mut self) -> T {
        // remove the entry from the heap
        let retval = self.heap_contents.swap_remove(0);
        // remove the entry in the index_by_id map
        self.index_by_id.remove(&retval.id);
        // fix up the heap
        self.heapify_down(0);
        println!("After get_min {:?}",self.heap_contents);
        *retval.data
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
            if child_index < self.heap_contents.len() {
                Some(child_index)
            }
            else {
                None
            }
    }

    fn get_right_child_index(&self, index: usize) -> Option<usize> {
            let child_index = index*2+2;
            if child_index < self.heap_contents.len() {
                Some(child_index)
            }
            else {
                None
            }
    }

    fn valid_index(&self, index: usize) -> bool {
        
        if index < self.heap_contents.len() {
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
            let return_value =	*self.heap_contents[a].data < *self.heap_contents[b].data;
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
            retval = *self.heap_contents[current_index].data < *self.heap_contents[parent_index].data;
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
            if *self.heap_contents[left_index.unwrap()].data <= *self.heap_contents[right_index.unwrap()].data {
                left_index
            }
            else {
                right_index
            }
        }

    }

    fn swap_with_parent(&mut self, cur_index: usize) {
        let parent_index = self.get_parent_index(cur_index).unwrap();
        let parent_id = self.heap_contents[parent_index].id.clone();
        let current_id = self.heap_contents[cur_index].id.clone();
        self.heap_contents.swap(cur_index,parent_index);
        self.index_by_id.insert(parent_id,cur_index);
        self.index_by_id.insert(current_id,parent_index);
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
                let child_id = self.heap_contents[smallest_child_index].id.clone();
                let current_id = self.heap_contents[cur_index].id.clone();
                self.heap_contents.swap(cur_index,smallest_child_index);

                // update the mapping from id to index to reflectx the new indexes
                self.index_by_id.insert(current_id,smallest_child_index.clone());
                self.index_by_id.insert(child_id,cur_index.clone());

                // after the swap, the current nodw will be where the child was
                // so continue the loop from there
                cur_index = smallest_child_index;
            }
            else {
                // child was NOT small, so we're donel
                break;
            }
        }
    }



}


#[cfg(test)]
mod minheap_tests {

    #[test]
    fn test1() {
        use crate::minheap::MinHeap;

        let mut v = MinHeap::<u32>::new();
        v.insert(61);
        v.insert(60);
        v.insert(50);
        v.insert(10);
        v.insert(18);
        v.insert(40);
        assert!(v.validate_heap())
    } 

    #[test]
    fn test2() {
        use crate::minheap::MinHeap;
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

        let mut v = MinHeap::<Person>::new();
        v.insert(Person { name: "Marvin".to_string(), age:61,  rank: 1});
        v.insert(Person { name: "Marvin".to_string(), age:60,  rank: 2});
        v.insert(Person { name: "Marcia".to_string(), age:50,  rank: 2});
        v.insert(Person { name: "Jordana".to_string(), age:10,  rank: 3});
        v.insert(Person { name: "Gizmo".to_string(), age:18,  rank:4 });
        assert!(v.validate_heap());
        assert_eq!(v.get_min().age,10);
        assert_eq!(v.get_min().age,18);
        assert_eq!(v.get_min().age,50);

    }

    #[test]
    fn test3() {
        use crate::minheap::MinHeap;

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

    }


    #[test]
    fn test4() {

        use crate::minheap::MinHeap;
        let mut v = MinHeap::<u32>::new();
        v.set(vec!(Box::new(1),Box::new(3),Box::new(2)));
        assert!(v.validate_heap());
        v.set(vec!(Box::new(3),Box::new(2),Box::new(1)));
        assert!(!v.validate_heap());
        v.set(vec!(Box::new(1),Box::new(5),Box::new(10),Box::new(3)));
        assert!(!v.validate_heap());
        v.set(vec!(Box::new(1),Box::new(5),Box::new(10),Box::new(7),Box::new(4)));
        assert!(!v.validate_heap());
        v.set(vec!(Box::new(1),Box::new(5),Box::new(10),Box::new(7),Box::new(11)));
        assert!(v.validate_heap());
        v.set(vec!(Box::new(1),Box::new(5),Box::new(10),Box::new(7),Box::new(11),Box::new(9)));
        assert!(!v.validate_heap());
        v.set(vec!(Box::new(1),Box::new(5),Box::new(10),Box::new(7),Box::new(11),Box::new(12),Box::new(6)));
        assert!(!v.validate_heap());
        v.set(vec!(Box::new(1),Box::new(5),Box::new(10),Box::new(7),Box::new(11),Box::new(12),Box::new(16)));
        assert!(v.validate_heap());

    }




}
