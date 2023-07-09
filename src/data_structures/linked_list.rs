#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(data: T) -> Self {
        Node { data, next: None }
    }
}

#[derive(Debug, Default)]
pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T: std::cmp::PartialEq + std::fmt::Debug> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList { head: None }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn append(&mut self, data: T) {
        let new_node = Box::new(Node::new(data));

        if self.is_empty() {
            self.head = Some(new_node);
        } else {
            let mut current = self.head.as_mut().unwrap();

            while current.next.is_some() {
                current = current.next.as_mut().unwrap();
            }

            current.next = Some(new_node);
        }
    }

    pub fn prepend(&mut self, data: T) {
        let mut new_node = Box::new(Node::new(data));
        new_node.next = self.head.take();
        self.head = Some(new_node);
    }

    pub fn delete(&mut self, data: T) {
        if self.is_empty() {
            return;
        }

        if self.head.as_ref().unwrap().data == data {
            self.head = self.head.as_mut().unwrap().next.take();
            return;
        }

        let mut current = self.head.as_mut().unwrap();

        while current.next.is_some() {
            if current.next.as_ref().unwrap().data == data {
                current.next = current.next.as_mut().unwrap().next.take();
                return;
            }
            current = current.next.as_mut().unwrap();
        }
    }

    pub fn search(&self, data: T) -> bool {
        let mut current = self.head.as_ref();

        while let Some(node) = current {
            if node.data == data {
                return true;
            }
            current = node.next.as_ref();
        }

        false
    }

    pub fn print_list(&self) {
        let mut current = self.head.as_ref();

        while let Some(node) = current {
            print!("{:?} ", node.data);
            current = node.next.as_ref();
        }
        println!();
    }
}