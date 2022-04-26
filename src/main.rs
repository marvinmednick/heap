mod minheap;
use crate::minheap::minheap::MinHeap;

fn main() {
    println!("Hello, world!");


    let mut v = MinHeap::<u32>::new();
    v.insert(61);
    v.insert(60);
    v.insert(50);
    v.insert(10);
    v.insert(18);
    v.insert(40);

    assert!(v.validate_heap())

}


#[cfg(test)]
mod tests {

    #[test]
    fn test1() {
        use crate::minheap::minheap::MinHeap;

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
        use crate::minheap::minheap::MinHeap;
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
        use crate::minheap::minheap::MinHeap;

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

        use crate::minheap::minheap::MinHeap;
        let mut v = MinHeap::<u32>::new();
        v.set(vec!(Box::new(1),Box::new(3),Box::new(2)));
        v.validate_heap();
        v.set(vec!(Box::new(3),Box::new(2),Box::new(1)));
        v.validate_heap();

    }



}
