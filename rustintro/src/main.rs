fn main() {
    println!("Hello, from the main.rs file!");
    get_height(180);
    human_identifier("Nastya", 39, 168.6);
    let x = {
        let price : i32 = 10;
        let tax : i32 = 2;
        price + tax
    };
    println!("{}", x);
    let added = add(1, 2);
    println!("addition is {}", added);
    let bmi = claculate_BMI(61.0, 1.68);
    println!("BMI is {:.2}", bmi);
}

fn get_height(height: i32) -> f64 {
    println!("Getting height: {}", height);
  return height as f64;
}

fn human_identifier(name: &str, age: u32, height: f32) {
   println!("My name is {}, and I am {} years old and {} cm tall", name, age, height);
}

const X : i32 = 10;

fn add(a: i32, b: i32) -> i32 {
    a + b
}

const fn claculate_BMI(weight: f32, height: f32) -> f32 {
    weight / (height * height)
}