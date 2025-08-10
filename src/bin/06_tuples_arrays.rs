fn main() {
    println!("Task 6: Tuples and Arrays");

    // TODO: Create a tuple with different data types
    // 1. Create a tuple named 'person' with (name: &str, age: i32, height: f64)
    // 2. Print the tuple using {:?} format
    // 3. Access individual elements using .0, .1, .2
    // 4. Destructure the tuple into separate variables
    let person = ("John", 25, 1.75);
    println!("{person:?}");

    // This will throw an error because tuple index access
    // isn't supported in format strings
    // println!("Name: {person.0}");

    println!("Name: {}", person.0);
    println!("Age: {}", person.1);
    println!("Height: {}", person.2);

    let (name, age, height) = person;
    println!("Name: {name}, Age: {age}, Height: {height}");

    // TODO: Work with arrays
    // 1. Create an array named 'numbers' with 5 integers
    // 2. Print the array using {:?} format
    // 3. Access elements by index (e.g., numbers[0])
    // 4. Use a for loop to iterate over the array
    // 5. Try to access an out-of-bounds index (this will cause a panic!)
    let numbers = [1, 2, 3, 4, 5];
    println!("{numbers:?}");
    println!("First number: {}", numbers[0]);
    for number in numbers {
        println!("{number}");
    }
    // println!("{}", numbers[10]); // This will cause a panic!

    // TODO: Array operations
    // 1. Create an array of 3 elements all initialized to 0
    // 2. Create an array of 4 elements all initialized to "hello"
    // 3. Print the length of an array using .len()
    let zeros = [0; 3];
    println!("{zeros:?}");
    println!("Length of zeros: {}", zeros.len());

    let hello = ["hello"; 4];
    println!("{hello:?}");
    println!("Length of hello: {}", hello.len());

    // TODO: Create a function that takes a tuple and returns the sum of numeric elements
    let person = ("John", 25, 1.75);
    let sum = sum_tuple_elements(person);
    println!("Sum of numeric elements: {sum}");
}

// TODO: Create a function that takes a tuple and returns the sum of numeric elements
fn sum_tuple_elements(t: (&str, i32, f64)) -> f64 {
    t.1 as f64 + t.2
}
