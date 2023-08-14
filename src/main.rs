use std::collections::HashMap;

struct Minimemcached {
    items: HashMap<String, i32>,
}

fn main() {
    let mut mm = Minimemcached {
        items: HashMap::new(),
    };

    mm.items.insert("first_key".to_string(), 42);
    mm.items.insert("second_key".to_string(), 123);

    println!("hello");
}
