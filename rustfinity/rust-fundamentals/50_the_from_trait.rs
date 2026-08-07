/*
    # 50. The From Trait

    The `From` trait is one of the most commonly used trait in the Rust programming language, **it is used for converting a value from one type to another.**

    After you implement the `From` trait for a type, the `Into` trait is implemented for you automatically. This means that you can then use `into()` on the source type to convert this type to the target type, and of course, you can use `From::from()` to convert from the source type to the target type.

    Here's an example on a use case for the `From` trait:

    ```rust
    fn main() {
        let f = Fahrenheit(32.0);
        let c: Celsius = f.into();
        println!("{}°F is {}°C", f.0, c.0);
    }
    ```

    This code works because the `Fahrenheit` struct implements the `From` trait for the `Celsius` struct. This allows us to convert a `Fahrenheit` instance to a `Celsius` instance using the `into()` method. The explicit type annotation `: Celsius` tells the compiler to turn it "into" a `Celsius` instance.

    ## Your Task

    Your task is to implement the `From` trait for the following struct types:

    ```rust
    struct Minutes(u32);
    struct Hours(u32);
    struct Days(u32);
    ```

    You need to implement the `From` trait for the following conversions:

    - `Minutes` to `Hours`
    - `Hours` to `Days`
    - `Minutes` to `Days`
    - `Days` to `Hours`
*/

pub struct Minutes(pub i32);
pub struct Hours(pub i32);
pub struct Days(pub i32);

impl From<Minutes> for Hours {
    fn from(minutes: Minutes) -> Hours {
        // Implement the minute to hour conversion here
        Hours(minutes.0 / 60)
    }
}

// TODO: implement from hours to days
impl From<Hours> for Days {
    fn from(hours: Hours) -> Days {
        Days(hours.0 / 24)
    }
}

// TODO: implement from minutes to days
impl From<Minutes> for Days {
    fn from(minutes: Minutes) -> Days {
        let hours: Hours = minutes.into();
        hours.into()
    }
}

// TODO: implement from days to hours
impl From<Days> for Hours {
    fn from(days: Days) -> Hours {
        Hours(days.0 * 24)
    }
}

// Example usage
pub fn main() {
    let minutes = Minutes(60);
    let hours: Hours = minutes.into();
    assert_eq!(hours.0, 1);
}
