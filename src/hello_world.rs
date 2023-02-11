use std::fmt::Debug;

pub fn hello_world() {
    println!("hello world");
}

pub fn hello_struct<T: Debug>(o: T) {
    println!("hello from: {:#?}", o)
}

pub fn hello_optional_struct<T: Debug>(o: Option<T>) {
    match o {
        Some(o) => hello_struct(o), // shadowing o...
        None => println!("Hey, there's nothing here..."),
    }
}

pub fn hello_struct_unless<T: Debug>(o: T) -> Result<(), String> {
    match format!("{:#?}", o).as_str() {
        "\"hello\"" => Err(String::from("You can't hello a hello!")),
        s => {
            println!("hello {s}");
            return Ok(());
        }
    }
}