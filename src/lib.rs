pub mod data_structures{

    pub mod linkedlist {

        /*
        implementations:
        initialize : new() 
        check for an empty list : is_empty() -> bool
        size : len() -> usize
        print_all_elements : traverse()
        iterate over each element : iter() // presently does not exist
        if the list contains element T : contains(&T)
        add_element_at_the_beginning : push_front(data)
        add_element_to_the_end : push_back(data)
        get the reference to the first element : front()
        get the reference to the last element : back()
        delete_first_element : pop_first() -> head
        delete_last_element : pop_last() -> tail
        remove the  value from the list : remove(&T) // presently does not exist
        clear the whole list : clear() -> String
        //maybe printing the whole liked list!!!!
        */

        //---------------------------------------------
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct Node<T> {
            pub data: T,
            pub next_node: Option<Box<Node<T>>>,
        }
        //---------------------------------------------
        #[derive(Debug, Clone)]
        pub struct Linkedlist<T> {
            pub head: Option<Box<Node<T>>>,
            pub length: usize,
        }
        //---------------------------------------------
        impl<T: std::fmt::Debug> Node<T> {
            
            pub fn new(data: T) -> Self {
                Self {
                    data: data,
                    next_node: None,
                }  
            }

            pub fn print(&self, index: u32) {
                println!("{} -> Data: {:?}, next-node-present: {:?}", index, self.data, self.next_node.is_some());
            }

        }
        //---------------------------------------------
        impl<T: std::fmt::Debug> Linkedlist<T> {
            
            pub fn new() -> Self {
                Linkedlist { 
                    head: None,
                    length: 0,
                }
            }

            pub fn is_empty(&self) -> bool {
                self.len() == 0
            }

            pub fn len(&self) -> usize {
                self.length
            }
            //Zero-based indexing.
            pub fn traverse(&self) -> () {
                let mut current_node = &self.head;
                let mut index = 0;
                while let Some(node) = current_node {
                    node.print(index);
                    current_node = &node.next_node;
                    index += 1;
                }
            }

            pub fn contains(&self , data: &T) -> bool where T: PartialEq{

                let mut current_node = self.head.as_ref();

                while current_node.is_some() {
                    
                    if &current_node.unwrap().data == data {
                        return  true;
                    }

                    current_node = current_node.unwrap().next_node.as_ref();
                }

                return  false;
            }

            pub fn push_front(&mut self, new_node_data: T) -> () {
                let mut new_node = Node { 
                    data: new_node_data,
                    next_node: None
                };
                self.length += 1;
                
                if self.head.is_none() {
                    ()
                } else {
                    let current_head = self.head.take().unwrap();
                    new_node.next_node = Some(current_head);
                }

                self.head = Some(Box::new(new_node));
                
            }
            
            pub fn push_back(&mut self, new_node_data: T) -> () {
                let new_node = Node { 
                    data: new_node_data, 
                    next_node: None
                };
                self.length += 1;  

                if self.head.is_none() {
                    self.head = Some(Box::new(new_node));
                    return;
                } else {

                    let mut current_node = self.head.as_mut().unwrap();

                    while current_node.next_node.is_some() {
                        current_node = current_node.next_node.as_mut().unwrap();
                    }

                    current_node.next_node = Some(Box::new(new_node));

                }

            }

            pub fn front(&self) -> Result<&T , &'static str> {

                if self.head.is_none() {
                    return Err("This list is empty");
                }

                Ok(&self.head.as_ref().unwrap().data)
                
            }

            pub fn back(&self) -> Result<&T , &'static str> {

                if self.head.is_none() {
                    return Err("The list is empty");
                }

                let mut current_node = self.head.as_ref().unwrap();

                while let Some(_node) = current_node.next_node.as_ref() {
                    current_node = current_node.next_node.as_ref().unwrap();
                }

                Ok(&current_node.as_ref().data)

            }

            pub fn pop_first(&mut self) -> Result<T , &'static str> {

                if self.head.is_none() {
                    return Err("Trying to pop_first element from an empty list");
                } else {
                    self.length -= 1;

                    let present_head = *self.head.take().unwrap();
                    self.head = present_head.next_node;
                    
                    return Ok(present_head.data);
                }

            } 

            pub fn pop_last(&mut self) -> Result<T , &'static str> {

                if self.head.is_none() {
                    return Err("Trying to pop_last element from an empty list");
                } else if  self.length == 1 {
                    //If only one node is present in the list.
                    self.length -= 1;

                    let present_head  = *self.head.take().unwrap();
                    return Ok(present_head.data);

                } else {
                    self.length -= 1;

                    let mut current_node = self.head.as_mut().unwrap();
                    //to set the current variable as the second last node.
                    while current_node.next_node.as_ref().unwrap().next_node.is_some() {
                        current_node = current_node.next_node.as_mut().unwrap();
                    }

                    let present_head  = current_node.next_node.take().unwrap();
                    return Ok(present_head.data);
                    
                }

            } 

            pub fn remove(&mut self, data : T) -> Option<T> where T : PartialEq{

                let mut current = &mut self.head;
                
                loop {
                    match current {
                        None => return None,
                        Some(node) if node.data == data => {
                            *current = node.next_node.take();
                            return Some(data);
                        },
                        Some(node) => {
                            current = &mut node.next_node;
                        }
                    }
                }

            }

 
            pub fn clear(&mut self) {

                //Deleting the firt element , creates a chain reaction to delete all the next elements one by one 'cause of no reference.
                if self.head.is_none() {
                    ()
                } else {
                    let _delete = self.head.take();
                    self.length = 0;
                }

            }

        }

    }

}


#[cfg(test)]

mod test{
    
    use crate::data_structures::linkedlist::Linkedlist;

    #[test]
    fn Linkedlist() {

        let mut tester: Linkedlist<u32> = Linkedlist::new();

        for yt in 13..19 {
            tester.push_front(yt);
        }

        tester.remove(15);

        tester.traverse();
        
    }

}