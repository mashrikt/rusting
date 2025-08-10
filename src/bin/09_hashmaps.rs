use std::collections::HashMap;

fn main() {
    println!("Task 9: HashMaps (Key-Value Pairs)");

    // TODO: HashMap creation
    // 1. Create an empty HashMap using HashMap::new()
    // 2. Create a HashMap with initial values using HashMap::from()
    // 3. Print the length using .len()
    // 4. Check if HashMap is empty using .is_empty()
    let mut hm: HashMap<&str, i32> = HashMap::new();
    let hm2: HashMap<&str, i32> = HashMap::from([("a", 1), ("b", 2), ("c", 3)]);
    println!("Length of hm: {}", hm.len());
    println!("Length of hm2: {}", hm2.len());
    println!("Is hm empty? {}", hm.is_empty());
    println!("Is hm2 empty? {}", hm2.is_empty());

    // TODO: Adding and updating entries
    // 1. Insert a key-value pair using .insert()
    // 2. Insert only if key doesn't exist using .entry().or_insert()
    // 3. Update an existing value using .entry().and_modify()
    // 4. Remove a key-value pair using .remove()
    hm.insert("a", 1);
    println!("{hm:?}");

    hm.entry("a").or_insert(9999);
    hm.entry("b").or_insert(2);
    hm.entry("c").or_insert(333);
    println!("{hm:?}");

    hm.entry("b").and_modify(|v| *v *= 11);
    println!("{hm:?}");

    hm.remove("a");
    println!("{hm:?}");


    // TODO: Accessing values
    // 1. Get a value using .get() (returns Option)
    // 2. Get a value with default using .get().unwrap_or()
    // 3. Check if key exists using .contains_key()
    // 4. Get all keys using .keys()
    // 5. Get all values using .values()
    let value = hm.get("a");
    println!("{value:?}");
    let value = hm.get("b");
    println!("{value:?}");

    let value = hm.get("a").unwrap_or(&1);
    println!("{value:?}");
    let value = hm.get("b").unwrap_or(&1);
    println!("{value:?}");

    let contains_a = hm.contains_key("a");
    println!("{contains_a}");
    let contains_b = hm.contains_key("b");
    println!("{contains_b}");

    let keys = hm.keys();
    println!("{keys:?}");
    let values = hm.values();
    println!("{values:?}");

    // TODO: HashMap iteration
    // 1. Iterate over key-value pairs using for loop
    // 2. Iterate over keys only
    // 3. Iterate over values only
    // 4. Iterate with mutable references using .iter_mut()
    for (key, value) in &hm {
        println!("Key: {key}, Value: {value}");
    }

    for key in hm.keys() {
        println!("Key: {key}");
    }

    for value in hm.values() {
        println!("Value: {value}");
    }

    for (_, value) in hm.iter_mut() {
        *value *= 10;
    }
    println!("{hm:?}");

    // TODO: HashMap operations
    // 1. Merge two HashMaps
    // 2. Clear all entries using .clear()
    // 3. Clone a HashMap using .clone()
    let hm3 = HashMap::from([("a", 1), ("b", 20), ("d", 4444)]);
    println!("{hm3:?}");
    hm.extend(hm3);
    println!("{hm:?}");

    println!("Complete the TODO comments above!");
    let text = "Hello, world! Hello, Rust!";
    let count = count_words(text);
    println!("count_words: {count:?}");

    let most_common = most_common_word(&count);
    println!("most_common: {most_common:?}");

}

// TODO: Create a function that counts word frequency in a string
fn count_words(text: &str) -> HashMap<String, usize> {
    let mut hm: HashMap<String, usize> = HashMap::new();
    for word in text.split_whitespace() {
        *hm.entry(word.to_string()).or_insert(0) += 1;
    }
    hm
}

// TODO: Create a function that finds the most common word
fn most_common_word(word_counts: &HashMap<String, usize>) -> Option<&String> {
    let mut max_count = 0;
    let mut most_common = None;
    for (word, count) in word_counts {
        if count > &max_count {
            max_count = *count;
            most_common = Some(word);
        }
    }
    most_common
}
