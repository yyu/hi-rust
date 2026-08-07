/*
    # 64. Box T

    The `Box<T>` type in Rust is a smart pointer that allows heap allocation for values, enabling efficient management of large data or data with unknown sizes at compile time.

    In this challenge, you'll work with a custom struct `Animal` and use `Box<T>` to manage its memory. You will also implement two functions to access the struct's fields by dereferencing the boxed instance.

    ## Your Task

    1. Implement the `create_animal` function to return a `Box<Animal>` containing a new `Animal` instance.

    2. Define another function, `access_animal`, that takes a `Box<Animal>` and returns a tuple `(String, u8)` representing the animal's name and age. Use dereferencing to access the fields.

    ## Hints

    <details>
      <summary>Click here to reveal hints</summary>

    - To create a boxed struct, use `Box::new(struct_instance)`.
    - Use the `*` operator to dereference the box and access its value.

    </details>
*/

pub struct Animal {
    pub name: String,
    pub age: u8,
}

pub fn create_animal(name: &str, age: u8) -> Box<Animal> {
    // Your code here
    Box::new(Animal {
        name: name.to_owned(),
        age
    })
}

pub fn access_animal(animal: Box<Animal>) -> (String, u8) {
    // Your code here
    (animal.name, animal.age)
}

// Example usage
pub fn main() {
    let animal = create_animal("Leo", 5);
    let (name, age) = access_animal(animal);
    println!("Animal's name: {}, age: {}", name, age);
}
