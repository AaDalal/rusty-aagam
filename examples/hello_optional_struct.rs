use std::io::stdin;

use rusty_aagam::hello_world::hello_optional_struct;

fn main () {    
    #[derive(Debug)]
    struct Tuple(i32, i32);
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u32
    }
    hello_optional_struct::<()>(None); // TODO: why do I need to specify the type here since it is unused anyways?
    hello_optional_struct(Some(Person{name: String::from("Aagam"), age: 20}));
    hello_optional_struct(Some(stdin()));
}