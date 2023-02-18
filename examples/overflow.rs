fn main () {
    let x: u8 = 1;
    let (x, ok) = x.overflowing_add(255);
    match x.checked_add(255) {
        Some(x) => (),
        None => (),
    }
}