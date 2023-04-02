#[derive(Debug, Clone)]
pub struct Node<T> {
    pub data: T,
    pub next_node: Option<Box<Node<T>>>,
}
#[derive(Debug, Clone)]
pub struct Linkedlist<U>{
    pub head: Option<Box<Node<U>>>,
    pub length: usize,
}

impl<T: std::fmt::Debug> Node<T> {
    //Creating a new node.
    pub fn new(data: T) -> Self {
        Self {
            data: data,
            next_node: None,
        }  
    }

    pub fn print(&self) -> () {
        println!("{:?}", self.data);
    }

}

impl<P: std::fmt::Debug> Linkedlist<P> {
    
    pub fn new() -> Self {
        Linkedlist { 
            head: None,
            length: 0,
        }
    }

    pub fn len(&self) -> &usize {
        &self.length
    }
    //This uses the std::fmt::Debug trait...
    /* fn traverse(&self) {
        let mut current_node = &self.head;
        while let Some(node_ref) = current_node {
            let node = &**node_ref;
            println!("{:?}",node.data);
            current_node = &node.next_node;
        }
    } */

    pub fn traverse(&self) -> () {
        let mut current_node = &self.head;
        while let Some(node) = current_node {
            node.print();
            current_node = &node.next_node;
        }
    }

    pub fn insert_start(&mut self,new_node_data: P) -> () {
        let mut new_node = Node { data: new_node_data, next_node: None};
        self.length += 1;
        //if the list is empty the inserted node at the starting will 
        //become the head, but if the list already contains some elements,
        //then firstly, the head will be taken into a variable and
        //the Linkedlist.head will be kept None,
        //it's box pointer will be stored in the new_node's next_node value
        //and then the new node will be the Linkedlist.head.
        if self.head.is_none() {
            ()
        } else {
            let current_head = self.head.take().unwrap();
            new_node.next_node = Some(current_head);
        }

        self.head = Some(Box::new(new_node));
        
    }
    //This function doesn't work.
    /* pub fn insert_end(&mut self, new_node_data: P) -> () {
        let mut new_node = Node { 
            data: new_node_data, 
            next_node: None
        };
        self.length += 1;  

        if self.head.is_none() {
            self.head = Some(Box::new(new_node));
        } else {
            let mut current_node = &self.head;

            while current_node.as_ref().unwrap().next_node.is_some() {
                current_node = &mut current_node.as_mut().unwrap().next_node;
            }


        } */
        
} 




#[cfg(test)]

mod test{
    use crate::Linkedlist;

    #[test]
    fn creating_the_node() {
        let mut hava = Linkedlist::new();
        hava.insert_start(3);
        hava.insert_start(6);
        for ty in 1..45 {
            hava.insert_start(ty);
        }
        hava.traverse();

        println!("This is the size of the linked list: {}",hava.len());
    }
}