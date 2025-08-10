fn main() {
    println!("Task 7: String Operations");

    // TODO: String creation and basic operations
    // 1. Create a String using String::new()
    // 2. Create a String using String::from("hello")
    // 3. Create a String using "hello".to_string()
    // 4. Print the length of the strings using .len()
    let s1 = String::new();
    let s2 = String::from("hello");
    let s3 = "hello".to_string();
    println!("Length of s1: {}", s1.len());
    println!("Length of s2: {}", s2.len());
    println!("Length of s3: {}", s3.len());

    // TODO: String manipulation
    // 1. Create a string and append to it using .push_str()
    // 2. Append a single character using .push()
    // 3. Convert a string to uppercase using .to_uppercase()
    // 4. Convert a string to lowercase using .to_lowercase()
    // 5. Trim whitespace using .trim()
    let mut s = String::from("Hello");
    s.push_str(", Rustacians");
    s.push('!');
    println!("{}", s);
    println!("{}", s.to_uppercase());
    println!("{}", s.to_lowercase());
    println!("{}", s.trim());

    // TODO: String slicing and iteration
    // 1. Create a string "Hello, World!"
    // 2. Slice the string to get "Hello" (first 5 characters)
    // 3. Slice the string to get "World" (characters 7-12)
    // 4. Iterate over characters using .chars()
    // 5. Iterate over bytes using .bytes()
    let s = "Hello, World!";
    let hello = &s[0..5];
    let world = &s[7..12];
    println!("{hello}");
    println!("{world}");
    for c in s.chars() {
        println!("{c}");
    }
    for b in s.bytes() {
        println!("{b}");
    }

    // TODO: String formatting
    // 1. Use format! macro to create a formatted string
    // 2. Use format! with multiple variables
    // 3. Use format! with named parameters
    let s1 = format!("Hello, {}!", "Rustacians");
    println!("{s1}");

    let name = "Alice";
    let age = 25;
    let s2 = format!("Name: {0}, Age: {1}", name, age);
    println!("{s2}");

    let s3 = format!("{lang} was released in {year}.", lang = "Rust", year = 2010);
    println!("{}", s3);

    // TODO: Create a function that reverses a string
    let s = "Hello, Rustacians!";
    let reversed = reverse_string(s);
    println!("{reversed}");

    let s = "Hello, Rustacians!";
    let count = count_vowels(s);
    println!("Number of vowels: {count}");
}

// TODO: Create a function that reverses a string
fn reverse_string(s: &str) -> String {
    let mut reversed = String::new();
    for c in s.chars().rev() {
        reversed.push(c);
    }
    reversed
}

// TODO: Create a function that counts vowels in a string
fn count_vowels(s: &str) -> usize {
    // Return the number of vowels (a, e, i, o, u) in the string
    let vowels = "aeiou";
    let mut count = 0;
    for c in s.chars() {
        if vowels.contains(c) {
            count += 1;
        }
    }
    count
}
