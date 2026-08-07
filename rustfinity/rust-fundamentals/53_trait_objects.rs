/*
    # 53. Trait Objects

    Now that we know a little bit about trait objects and how to work with them, let's use them in a different example, this challenge focuses on using `Box<dyn Trait>` as fields in structs to encapsulate objects with dynamic behavior.

    ## Your Task

    You need to define a trait called `Renderable` and create two structs, `Circle` and `Rectangle`, that implement this trait. Then, you will create another struct called `Canvas`, which can hold a collection of objects implementing the `Renderable` trait. The `Canvas` struct should have methods to add objects and render all objects.

    ### Requirements

    Here are the requirements for the program, make sure to read them carefully:

    #### Define the `Renderable` Trait

    - The trait should have a method `render(&self) -> String` to represent the object visually.

    #### Define the `Circle` and `Rectangle` Structs

    - `Circle` should have a `radius: f64` field.
    - `Rectangle` should have `width: f64` and `height: f64` fields.

    #### Implement the `Renderable` Trait

    - For `Circle`, the `render` method should return a string like `"Circle with radius X"`.
    - For `Rectangle`, the `render` method should return a string like `"Rectangle with width X and height Y"`.

    #### Define the `Canvas` Struct

    - The struct should have a `shapes` field that can store a `Vec` of the Renderable trait objects.
    - Implement the following methods for `Canvas`:
      - `new() -> Canvas`: Initializes an empty canvas.
      - `add_shape()`: Adds a shape to the canvas.
      - `render_all()`: Returns a vector of strings, each representing a rendered shape.

    **⚠️ Make sure you make all the relevant items public with the `pub` keyword, this is required for the tests to work.**

    ## Hints

    If you're stuck, here are some hints to help you solve the challenge:

    <details>
      <summary>Click here to reveal hints</summary>

    - Use `Box<dyn Trait>` to store objects with dynamic behavior. e.g.
      ```rust
      pub struct Canvas {
        pub shapes: Vec<Box<dyn Renderable>>,
      }
      ```

    </details>
*/

// 1. Create the Renderable trait
pub trait Renderable {
    fn render(&self) -> String;
}

// 2. Create the Circle and Rectangle structs
pub struct Circle {
    pub radius: f64
}

pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

// 3. Implement the trait for Circle and Rectangle
impl Renderable for Circle {
    fn render(&self) -> String {
        format!("Circle with radius {}", self.radius)
    }
}

impl Renderable for Rectangle {
    fn render(&self) -> String {
        format!("Rectangle with width {} and height {}", self.width, self.height)
    }
}

// 4. Create the Canvas struct
pub struct Canvas {
    pub shapes: Vec<Box<dyn Renderable>>
}

// 5. Implement the Canvas struct
impl Canvas {
    pub fn new() -> Self {
        Self {
            shapes: vec![]
        }
    }

    pub fn add_shape(&mut self, shape: Box<dyn Renderable>) {
        self.shapes.push(shape)
    }

    pub fn render_all(&self) -> Vec<String> {
        self.shapes.iter().map(|s| s.render()).collect()
    }
}

// Example usage
pub fn main() {
    let mut canvas = Canvas::new();
    canvas.add_shape(Box::new(Circle { radius: 5.0 }));
    canvas.add_shape(Box::new(Rectangle {
        width: 3.0,
        height: 4.0,
    }));
    let rendered_shapes = canvas.render_all();
    for shape in rendered_shapes {
        println!("{}", shape);
    }
}
