use std::io::stdin;

use rusty_aagam::hello_world::hello_struct_unless;

fn main () {    
    #[derive(Debug)]
    struct Tuple(i32, i32);
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u32
    };
    hello_struct_unless("hello".into());
    hello_struct_unless(Tuple(0,0)).expect("Error!");
    hello_struct_unless(Person{name: String::from("Aagam"), age: 20}).expect("Error!");
    hello_struct_unless("hello").expect("Error!"); // errors!
}