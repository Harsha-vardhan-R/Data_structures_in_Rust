#[derive(Debug)]
pub struct Node<T> {
    pub data: T,
    pub next_node: Option<Box<Node<T>>>,
}

pub struct Linkedlist<U>{
    pub head: Option<Box<Node<U>>>,
    pub length: usize,
}

impl<T> Node<T> {
    //Creating a new node.
    pub fn new(data: T) -> Self {
        Self {
            data: data,
            next_node: None,
        }  
    }

}


impl<P> Linkedlist<P> {
    
    pub fn new() -> Self {
        Linkedlist { 
            head: None,
            length: 0,
        }
    }

    pub fn len(&self) -> &usize {
        &self.length
    }

    pub fn insert_start(&mut self,new_node_data: P) {
        let mut new_node = Node { data: new_node_data, next_node: None};
        self.length += 1;
        //if the list is empty the inserted node at the starting will 
        //become the head, but if the list already contains some elements,
        //then firstly, the head will be taken into a variable and
        //the Linkedlist.head will be kept None,
        //it's box pointer will be stored in the new_node's next_node value
        //and then the new node will be the Linkedlist.head.
        if self.length == 0 {
            ()
        } else {
            let current_head = self.head.take().unwrap();
            new_node.next_node = Some(current_head);
        }

        self.head = Some(Box::new(new_node));
        
    }
}



#[cfg(test)]

mod test{
    #[test]
    fn creating_the_node() {
        assert_eq!(8, 4*2);
    }
}