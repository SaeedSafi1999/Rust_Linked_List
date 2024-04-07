mod Linked_List;

use Linked_List::List;



#[test]
fn test_list() {
    ///struct example
    #[derive(Debug)]
    struct Person {
        name: String,
        age: i32,
    }

    let mut list = List::new();
    let mut persons = List::new();

    persons.add(Person { name: "saeed".to_string(), age: 25 });
    persons.add(Person { name: "reza".to_string(), age: 30 });
    persons.add(Person { name: "ali".to_string(), age: 45 });

    println!("persons before remove array:{:?}", persons.to_array());

    persons.remove_index(1);

    println!("persons after remove array:{:?}", persons.to_array());

    match persons.get_by_index(1) {
        Some(z) => println!("{:?}", z),
        None => println!("not found"),
    }

    /// i32 example
    list.add(1);
    list.add(2);
    list.add(4);
    list.add(7);
    list.add(89);

    println!("First: {:?}", list.first());
    println!("Last: {:?}", list.last());
    println!("Index of 2: {:?}", list.get_index(&2));
    println!("Next after 2: {:?}", list.next(&2));
    println!("Is 3 last index: {:?}", list.is_last_index(&3));
    println!("Is 3 last index: {:?}", list.is_last_index(&89));
    println!("Array: {:?}", list.to_array());
}
