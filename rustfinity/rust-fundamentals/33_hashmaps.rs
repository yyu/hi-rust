/*
    # 33. Hashmaps

    Hashmaps are one of the most versatile data structures in Rust, providing a way to store key-value pairs. They are part of the standard library and offer a convenient way to associate values with unique keys. In this challenge, you will implement a simple key-value store using Rust's `HashMap`.

    The `HashMap` type is part of the `std::collections` module and allows you to perform operations such as inserting, retrieving, and removing elements efficiently.

    ## Your Task

    You are tasked with implementing two functions for a key-value store:

    1. `insert_or_update`: Inserts a new key-value pair into the `HashMap`, or updates the value if the key already exists.
    2. `get_value`: Retrieves a value by its key. If the key is not present, return `None`.

    You will write both functions to work with a `HashMap<String, String>`.

    ### Requirements

    #### insert_or_update

    - Accepts a mutable reference to a `HashMap<String, String>`, a key of type `String`, and a value of type `String`.
    - If the key exists in the map, updates its value.
    - If the key does not exist, inserts the new key-value pair.

    #### get_value

    - Accepts an immutable reference to a `HashMap<String, String>` and a key of type `String`.
    - Returns an `Option<String>`:
      - `Some(value)` if the key is present.
      - `None` if the key is absent.

    ## Hints

    <details>
    <summary>Click here to reveal hints</summary>

    - Use the `HashMap` type from `std::collections`.
    - To insert or update a value, use the `.insert()` method of `HashMap`.
    - To retrieve a value by its key, use the `.get()` method of `HashMap`.

    </details>
*/

use std::collections::HashMap;

/// Inserts a key-value pair into the hashmap or updates the value if the key exists.
pub fn insert_or_update(map: &mut HashMap<String, String>, key: String, value: String) {
    // Your code here...
    map.insert(key, value);
}

/// Retrieves the value associated with a key from the hashmap.
pub fn get_value(map: &HashMap<String, String>, key: String) -> Option<String> {
    // Your code here...
    map.get(&key).cloned()
}

// Example usage
pub fn main() {
    let mut store = HashMap::new();

    // Insert a new key-value pair
    insert_or_update(&mut store, "name".to_string(), "Alice".to_string());

    // Update an existing key
    insert_or_update(&mut store, "name".to_string(), "Bob".to_string());

    // Retrieve the value by key
    let value = get_value(&store, "name".to_string());
    assert_eq!(value, Some("Bob".to_string()));

    // Try to get a non-existent key
    let missing = get_value(&store, "age".to_string());
    assert_eq!(missing, None);
}
