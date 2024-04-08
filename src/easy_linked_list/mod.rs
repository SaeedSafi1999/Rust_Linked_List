struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(data: T) -> Self {
        Node { data, next: None }
    }
}

///this is unsize generic list for stored any type
///
///
/// # Examples
///
/// ```
///#[derive(Debug)] //struct example
///struct Person {
///    name: String,
///    age: i32,
///}
///let mut persons = List::new(); //declare package
///
///persons.add(Person { name: "saeed".to_string(), age: 25 });// set element
///
/// ```
pub struct List<T> {
    head: Option<Box<Node<T>>>,
}

///this is unsize generic list for stored any type
///
///
/// # Examples
///
/// ```
///#[derive(Debug)] //struct example
///struct Person {
///    name: String,
///    age: i32,
///}
///let mut persons = List::new(); //declare package
///
///persons.add(Person { name: "saeed".to_string(), age: 25 });// set element
///
/// ```
impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    ///this method used for insert in the list
    ///
    ///
    /// # Examples
    ///
    /// ```
    ///#[derive(Debug)] //struct example
    ///struct Person {
    ///    name: String,
    ///    age: i32,
    ///}
    ///let mut persons = List::new(); //declare package
    ///
    ///persons.add(Person { name: "saeed".to_string(), age: 25 });// insert element
    ///
    /// ```
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

    ///this method used for remove all elements in the list
    ///
    ///
    /// # Examples
    ///
    /// ```
    ///#[derive(Debug)] //struct example
    ///struct Person {
    ///    name: String,
    ///    age: i32,
    ///}
    ///let mut persons = List::new(); //declare package
    ///
    ///persons.clear();// remove all elements
    ///
    /// ```
    pub fn clear(&mut self) {
        self.head = None;
    }

    ///this method used for insert in the list
    ///
    ///# Arguments
    ///
    /// * `index` - index of the value you want to get from list.
    ///
    ///
    /// # Examples
    ///
    /// ```
    ///#[derive(Debug)] //struct example
    ///struct Person {
    ///    name: String,
    ///    age: i32,
    ///}
    ///let mut persons = List::new(); //declare package
    ///
    ///persons.add(Person { name: "saeed".to_string(), age: 25 });// set element index=>0
    ///persons.add(Person { name: "safi".to_string(), age: 26 });// set element index=>1
    ///
    ///persons.get_by_index(1);// get element by index
    ///
    /// ```
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

    ///this method used for remove element with specefic index
    ///
    ///# Arguments
    ///
    /// * `index` - index of the value you want to get from list.
    ///
    ///
    /// # Examples
    ///
    /// ```
    ///#[derive(Debug)] //struct example
    ///struct Person {
    ///    name: String,
    ///    age: i32,
    ///}
    ///let mut persons = List::new(); //declare package
    ///
    ///persons.add(Person { name: "saeed".to_string(), age: 25 });// set element index=>0
    ///persons.add(Person { name: "safi".to_string(), age: 26 });// set element index=>1
    ///
    ///persons.remove_index(1);// remove element by index
    ///
    /// ```
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

    ///this method used for get first element in the list
    ///
    ///
    /// # Examples
    ///
    /// ```
    ///#[derive(Debug)] //struct example
    ///struct Person {
    ///    name: String,
    ///    age: i32,
    ///}
    ///let mut persons = List::new(); //declare package
    ///
    ///persons.add(Person { name: "saeed".to_string(), age: 25 });// set element
    ///persons.add(Person { name: "safi".to_string(), age: 26 });// set element
    ///
    ///persons.first();// return first element inthe list
    ///
    /// ```
    pub fn first(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.data)
    }

    ///this method used for get last element in the list
    ///
    ///
    /// # Examples
    ///
    /// ```
    ///#[derive(Debug)] //struct example
    ///struct Person {
    ///    name: String,
    ///    age: i32,
    ///}
    ///let mut persons = List::new(); //declare package
    ///
    ///persons.add(Person { name: "saeed".to_string(), age: 25 });// set element
    ///persons.add(Person { name: "safi".to_string(), age: 26 });// set element
    ///
    ///persons.last();// return last element inthe list
    ///
    /// ```
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

    ///this method used for get index of element in the list
    ///
    ///# Arguments
    ///
    /// * `value` - value type that you gave to list<T:as you passed> .
    ///
    ///
    /// # Examples
    ///
    /// ```
    ///let mut list = List::new(); //declare package
    ///
    /// // i32 example
    ///list.add(1);
    ///list.add(2);
    ///list.add(4);
    ///
    ///println!("Index of 2: {:?}", list.get_index(&2)); //return 1
    /// ```
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

    ///this method used for get next element in the list
    ///
    ///# Arguments
    ///
    /// * `value` - value type that you gave to list<T:as you passed> .
    ///
    ///
    /// # Examples
    ///
    /// ```
    ///let mut list = List::new(); //declare package
    ///
    /// // i32 example
    ///list.add(1);
    ///list.add(2);
    ///list.add(4);
    ///
    /// println!("Next after 2: {:?}", list.next(&2)); //return 4
    /// ```
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

    ///this method used for find ount an element is the last of elemnts in the list or not
    ///
    ///# Arguments
    ///
    /// * `value` - value type that you gave to list<T:as you passed> .
    ///
    ///
    /// # Examples
    ///
    /// ```
    ///let mut list = List::new(); //declare package
    ///
    /// // i32 example
    ///list.add(1);
    ///list.add(2);
    ///list.add(4);
    ///
    /// println!("Is 3 last index: {:?}", list.is_last_index(&4)); //return true
    /// ```
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

    ///return simple rust array 
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
