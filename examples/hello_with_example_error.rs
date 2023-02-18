

#[derive(Debug)]
struct ExampleError {
    error: String
}

impl From<String> for ExampleError {
    fn from(value: String) -> Self {
        ExampleError { error: value }
    }
}

fn main() -> Result<(), ExampleError> {
    let rand_val: i32 = rand::random();
    if rand_val % 2 == 0 {
        Result::Ok(())
    } else {
        Result::Err(
            ExampleError::from(String::from("That's odd..."))
        )
    }
}