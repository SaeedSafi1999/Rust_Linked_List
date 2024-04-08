

pub struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(data: T) -> Self {
        Node { data, next: None }
    }
}

pub struct List<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn add(&mut self, data: T) {
        let new_node = Box::new(Node::new(data));
        match self.head {
            Some(ref mut head) => {
                let mut current = head;
                while let Some(ref mut next_node) = current.next {
                    current = next_node;
                }
                current.next = Some(new_node);
            }
            None => {
                self.head = Some(new_node);
            }
        }
    }

    pub fn clear(&mut self) {
        self.head = None;
    }

    pub fn get_by_index(&self, index: usize) -> Option<&T> {
        let mut current = &self.head;
        let mut current_index = 0;
        while let Some(ref node) = *current {
            if current_index == index {
                return Some(&node.data);
            }
            current_index += 1;
            current = &node.next;
        }
        None
    }

    pub fn remove_index(&mut self, index: usize) -> Option<T> {
        let mut current = &mut self.head;
        for _ in 0..index - 1 {
            if let Some(ref mut node) = *current {
                current = &mut node.next;
            } else {
                return None;
            }
        }
        current.as_mut().and_then(|node| {
            node.next.take().map(|next_node| {
                let next_data = next_node.data;
                node.next = next_node.next;
                next_data
            })
        })
    }

    pub fn first(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.data)
    }

    pub fn last(&self) -> Option<&T> {
        let mut current = &self.head;
        while let Some(ref node) = *current {
            if node.next.is_none() {
                return Some(&node.data);
            }
            current = &node.next;
        }
        None
    }

    pub fn get_index(&self, value: &T) -> Option<usize> where T: PartialEq {
        let mut current = &self.head;
        let mut index = 0;
        while let Some(ref node) = *current {
            if &node.data == value {
                return Some(index);
            }
            index += 1;
            current = &node.next;
        }
        None
    }

    pub fn next(&self, value: &T) -> Option<&T> where T: PartialEq {
        let mut current = &self.head;
        while let Some(ref node) = *current {
            if &node.data == value {
                return node.next.as_ref().map(|next_node| &next_node.data);
            }
            current = &node.next;
        }
        None
    }

    pub fn is_last_index(&self, value: &T) -> bool where T: PartialEq {
        if let Some(index) = self.get_index(value) {
            let mut current = &self.head;
            let mut i = 0;
            while let Some(ref node) = *current {
                if i == index {
                    return node.next.is_none();
                }
                i += 1;
                current = &node.next;
            }
        }
        false
    }


    pub fn to_array(&self) -> Vec<&T> {
        let mut array = Vec::new();
        let mut current = &self.head;
        while let Some(ref node) = *current {
            array.push(&node.data);
            current = &node.next;
        }
        array
    }
}