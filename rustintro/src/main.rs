fn main() {
    println!("Hello, from the main.rs file!");
    get_height(180);
    human_identifier("Nastya", 39, 168.1);
}

fn get_height(height: i32) -> f64 {
    println!("Getting height: {}", height);
  return height as f64;
}

fn human_identifier(name: &str, age: u32, height: f32) {
   println!("My name is {}, and I am {} years old and {} cm tall", name, age, height);
}