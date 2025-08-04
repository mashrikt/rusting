fn main() {
    println!("Task 3: Functions");

    // TODO: Call the functions below and print their results
    // greet("Rust Learner");
    // let sum = add(5, 3);
    // let product = multiply(4, 7);

    greet("Rust Learner");
    let sum = add(5, 3);
    println!("Sum: {}", sum);
    let product = multiply(4, 7);
    println!("Product: {}", product);
}

// TODO: Implement these functions
// 1. Create a function 'greet' that takes a string parameter and prints "Hello, {name}!"
// 2. Create a function 'add' that takes two integers and returns their sum
// 3. Create a function 'multiply' that takes two integers and returns their product

fn greet(name: &str) {
    println!("Hello, {}!", name);
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn multiply(a: i32, b: i32) -> i32 {
    a * b
}
