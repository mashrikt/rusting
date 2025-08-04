fn main() {
    println!("Task 5: Ownership");

    // TODO: Create a String variable and understand ownership
    // 1. Create a String variable named 'text' with value "Hello Rust"
    // 2. Print the text
    // 3. Call a function that takes ownership of the string
    // 4. Try to print the text again (this should cause a compilation error)

    let text = String::from("Hello Rust");
    println!("{text} from main");
    take_ownership(text);
    // This line will cause a compilation error
    // println!("{text}");

    // TODO: Use borrowing instead
    // 1. Create another String variable
    // 2. Call a function that borrows the string (uses &)
    // 3. Print the string after the function call (this should work)

    let text = String::from("Hello Rust");
    borrow_string(&text);
    println!("{text} from main again");
}

// TODO: Implement these functions
// 1. A function that takes ownership of a String
// 2. A function that borrows a String reference
fn take_ownership(text: String) {
    println!("{text} from take_ownership");
}

fn borrow_string(text: &String) {
    println!("{text} from borrow_string");
}
