fn main() {
    let x = 5;
    let x: i32 = 10; // shadowing
    x = 100; // errors...
    let mut x = 15;
    x = 100;
}