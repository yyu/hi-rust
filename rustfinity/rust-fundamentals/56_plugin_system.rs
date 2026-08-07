/*
    # 56. Plugin System

    We now have an overview of traits, how to define them and how they work, now it's time to put that knowledge to use and build a plugin system using traits.

    A plugin in this system is any type that implements a specific trait. Each plugin will perform a specific task, and the system should manage a collection of these plugins, executing them in sequence. You’ll also address advanced issues like object safety and resolving potential conflicts between overlapping trait implementations.

    ## Your Task

    Design and implement a plugin system using trait objects. You will:

    1. Define a `Plugin` trait that includes methods for initialization and execution.
    2. Create a `PluginManager` struct to manage plugins. It should:
       - Dynamically load plugins implementing the `Plugin` trait.
       - Allow adding and removing plugins.
       - Execute all registered plugins in sequence.

    ### Requirements

    - The `Plugin` trait should include the following methods:
      - `fn name(&self) -> &str;` - Returns the name of the plugin.
      - `fn execute(&self);` - Executes the plugin's functionality.
    - The `PluginManager` should:
      - Have the following methods and associated functions:
        - `new() -> Self` - Creates a new `PluginManager` instance.
        - `add_plugin` - Adds a plugin to the list.
        - `remove_plugin` - Removes a plugin from the list and returns the removed plugin if found.
        - `execute_all` - Executes all registered plugins.
    - If a duplicate plugin is added (with the same name), it should **panic** with the message "Plugin with name [name] already exists".

    Make sure you make all relevant items public.

    ## Hints

    <details>
        <summary>Click here to reveal hints</summary>

    - **Dynamic Dispatch**: Store plugins in a `Vec<Box<dyn Plugin>>` for dynamic dispatch.
    - **Plugin Uniqueness**: Use the `name` method to identify and ensure uniqueness among plugins.

    </details>
*/

use std::collections::HashMap;

pub trait Plugin {
    // 1. Finish the trait
    fn name(&self) -> &str;
    fn execute(&self);
}

pub struct PluginManager {
    // 2. Finish the struct
    // Make fields public
    pub plugins: HashMap<String, Box<dyn Plugin>>
}

// 3. Implement the PluginManager
impl PluginManager {
    pub fn new() -> Self {
        Self {
            plugins: HashMap::new()
        }
    }

    pub fn add_plugin(&mut self, p: Box<dyn Plugin>) {
        let new_plugin_name: String = p.name().to_owned();
        if self.plugins.contains_key(&new_plugin_name) {
            panic!("Plugin with name '{}' already exists", new_plugin_name);
        }
        self.plugins.insert(new_plugin_name, p);
    }

    pub fn remove_plugin(&mut self, plugin_name: &str) -> Option<Box<dyn Plugin>> {
        self.plugins.remove(plugin_name)
    }

    pub fn execute_all(&self) {
        for (_, v) in &self.plugins {
            v.execute();
        }
    }
}

// Example usage
pub struct MyPlugin;

impl Plugin for MyPlugin {
    fn name(&self) -> &str {
        "MyPlugin"
    }
    fn execute(&self) {
        println!("Executing MyPlugin");
    }
}

impl MyPlugin {
    fn new() -> Self {
        Self
    }
}

pub fn main() {
    let mut manager = PluginManager::new();

    manager.add_plugin(Box::new(MyPlugin::new()));
    manager.execute_all();
}
