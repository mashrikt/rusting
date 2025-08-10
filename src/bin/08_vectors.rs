fn main() {
    println!("Task 8: Vectors (Dynamic Arrays)");

    // TODO: Vector creation
    // 1. Create an empty vector using Vec::new()
    // 2. Create a vector with initial values using vec![1, 2, 3]
    // 3. Create a vector with repeated values using vec![0; 5]
    // 4. Print the length and capacity using .len() and .capacity()
    let v1: Vec<i32> = Vec::new();
    let v2 = vec![1, 2, 3];
    let v3 = vec![0; 5];
    println!("Length of v1: {}", v1.len());
    println!("Length of v2: {}", v2.len());
    println!("Length of v3: {}", v3.len());
    println!("Capacity of v1: {}", v1.capacity());
    println!("Capacity of v2: {}", v2.capacity());
    println!("Capacity of v3: {}", v3.capacity());

    // TODO: Adding and removing elements
    // 1. Add elements to the end using .push()
    // 2. Remove the last element using .pop()
    // 3. Insert an element at a specific index using .insert()
    // 4. Remove an element at a specific index using .remove()
    let mut v = vec![1, 2, 3];
    v.push(4);
    println!("{v:?}");
    v.pop();
    println!("{v:?}");
    v.insert(1, 0);
    println!("{v:?}");
    v.remove(1);
    println!("{v:?}");

    // TODO: Vector iteration
    // 1. Iterate over vector elements using for loop
    // 2. Iterate with index using .iter().enumerate()
    // 3. Use .iter() to borrow elements
    // 4. Use .iter_mut() to get mutable references
    let mut numbers = vec![10, 20, 30, 40, 50];

    for num in &numbers {
        println!("{num}");
    }

    for (i, num) in numbers.iter().enumerate() {
        println!("Index: {i}, Value: {num}");
    }

    // borrow elements (non-mutable)
    let sum: i32 = numbers.iter().sum();
    println!("Sum: {}", sum);

    for num in numbers.iter_mut() {
        *num += 1; // modify in place
    }
    println!("{v:?}");


    // TODO: Vector operations
    // 1. Check if vector contains an element using .contains()
    // 2. Find the index of an element using .iter().position()
    // 3. Sort a vector using .sort()
    // 4. Reverse a vector using .reverse()
    let mut numbers = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
    if numbers.contains(&5) {
        println!("Vector contains 5");
    }
    let index = numbers.iter().position(|x| *x == 5);
    println!("Index of 5: {}", index.unwrap());
    numbers.sort();
    println!("{numbers:?}");
    numbers.reverse();
    println!("{numbers:?}");

    // TODO: Vector slicing
    // 1. Create a slice of the first 3 elements
    // 2. Create a slice of the last 3 elements
    // 3. Create a slice from index 1 to 4
    let slice1 = &numbers[..3];
    println!("{slice1:?}");
    let slice2 = &numbers[7..];
    println!("{slice2:?}");
    let slice3 = &numbers[1..4];
    println!("{slice3:?}");

    println!("Complete the TODO comments above!");
    let max = find_max(&numbers);
    println!("Max: {}", max.unwrap());
    let unique = remove_duplicates(&numbers);
    println!("{unique:?}");
}

// TODO: Create a function that finds the maximum value in a vector
fn find_max(numbers: &[i32]) -> Option<i32> {
    let max = numbers.iter().copied().max();
    max
}


// TODO: Create a function that removes duplicates from a vector
fn remove_duplicates(numbers: &[i32]) -> Vec<i32> {
    // Return a new vector with duplicates removed
    let mut unique = Vec::new();
    for num in numbers {
        if !unique.contains(num) {
            unique.push(*num);
        }
    }
    unique
}
