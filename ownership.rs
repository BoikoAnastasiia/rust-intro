fn main() {
    let s = String::from("Star");
    let length = calculate_length(&s);
    println!("The length of '{}' is {}.", s, length);
    increment();
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn increment () {
    let mut x: i32 = 5;
    let y = &mut x;
    *y += 1;
    println!("x is now {}", x);
}