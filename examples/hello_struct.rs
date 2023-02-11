use std::io::stdin;

use rusty_aagam::hello_world::hello_struct;

fn main () {    
    #[derive(Debug)]
    struct Tuple(i32, i32);
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u32
    };
    hello_struct(Tuple(0,0));
    hello_struct(Person{name: String::from("Aagam"), age: 20});
    hello_struct(stdin());
}