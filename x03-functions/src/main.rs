fn main() {
    hello_world();
    tell_height(182);
    human_id("Gideon", 25, 182.0);

    let x = {
        let price: i32 = 5;
        let qty = 10;
        price * qty
    };

    println!("Result is: {}", x);

    let result: i32 = add(4, 80);
    println!("add function result is: {}", result);

    let weight = 70.0;
    let height = 1.82;
    let bmi = bmi(weight, height);

    println!("Your BMI is: {:.2}", bmi);
}

//Hoisiting - can call function anywhere in your code
fn hello_world(){
    println!("Hello, Rust 🦀!");
}

//you can insert input values
fn tell_height(height: u32){
    println!("My height is {} cm.", height);
}

fn human_id(name: &str, age: u32, height: f32){
    println!("My name is {}, I am {} years old, and my height is {} cm.",
            name, age, height);
}

fn add(a: i32, b: i32) -> i32{
    a + b
}

fn bmi(weight: f32, height: f32)->f32{
    weight / (height * height)
}