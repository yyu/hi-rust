/*
    # 29. Methods On Enums

    Enums in Rust can have associated functions and methods, just like structs. These methods make it easier to encapsulate behavior directly within the enum.

    In this challenge, you'll model the statuses of different vehicles and implement methods to describe their behavior.

    ## Your Task

    Create an enum `VehicleStatus` with the following variants:

    1. `Parked` — a unit variant representing a parked vehicle.
    2. `Driving { speed: u32 }` — a named field variant representing a vehicle driving at a certain speed.
    3. `BrokenDown(String)` — a tuple variant with a `String` describing the reason for the breakdown.

    Implement the following methods for `VehicleStatus`:

    - `is_operational(&self) -> bool`:
      - Returns `true` if the vehicle is either `Parked` or `Driving`.
      - Returns `false` if the vehicle is `BrokenDown`.

    - `description(&self) -> String`:
      - Returns `"The vehicle is parked."` for `Parked`.
      - Returns `"The vehicle is driving at {speed} km/h."` for `Driving { speed }`.
      - Returns `"The vehicle is broken down: {reason}."` for `BrokenDown(reason)`.

    ## Hints

    <details>
    <summary>Click here to reveal hints</summary>

    - Use a `match` expression inside the methods to handle each variant.
    - Remember to use `&self` for the methods since they should not consume the enum.
    - Use `format!` to construct strings with dynamic values, such as `speed` and `reason`.

    </details>
*/

pub enum VehicleStatus {
    // Define the VehicleStatus variants here
    Parked,
    Driving { speed: u32 },
    BrokenDown(String),
}

impl VehicleStatus {
    pub fn is_operational(&self) -> bool {
        // Your code here...
        match self {
            Self::BrokenDown(_) => false,
            _ => true,
        }
    }

    pub fn description(&self) -> String {
        // Your code here...
        match self {
            Self::Parked => "The vehicle is parked.".to_string(),
            Self::Driving { speed } => format!("The vehicle is driving at {speed} km/h."),
            Self::BrokenDown(reason) => format!("The vehicle is broken down: {reason}."),
        }
    }
}

// Example use case
pub fn main() {
    let parked = VehicleStatus::Parked;
    assert!(parked.is_operational());
    assert_eq!(parked.description(), "The vehicle is parked.");

    let driving = VehicleStatus::Driving { speed: 80 };
    assert!(driving.is_operational());
    assert_eq!(driving.description(), "The vehicle is driving at 80 km/h.");

    let broken_down = VehicleStatus::BrokenDown("Flat tire".to_string());
    assert!(!broken_down.is_operational());
    assert_eq!(
        broken_down.description(),
        "The vehicle is broken down: Flat tire."
    );
}
