use std::any::Any;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

trait RegistryItem: Any {
    fn as_any(&self) -> &dyn Any;
}

impl<T: 'static> RegistryItem for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

// Define the struct for the registry
// pub struct Registry {
//     data: Mutex<HashMap<String, Box<dyn RegistryItem>>>,
// }

// Define a registry to store and manage the variations
pub struct Registry {
    items: HashMap<String, Box<dyn super::template::Operator>>,
}

// Implement methods for the registry
impl Registry {
    pub fn new() -> Self {
        Registry {
            items: HashMap::new(),
        }
    }

    pub fn register(&mut self, item: Box<dyn super::template::Operator>) {
        println!("Registered {}", item.get_name().to_string());
        self.items.insert(item.get_name().to_string(), item);
    }

    pub fn get(&self, name: &str) -> Option<&Box<dyn super::template::Operator>> {
        self.items.get(name)
    }

    // pub fn new() -> Arc<Self> {
    //     Arc::new(Self {
    //         items: Mutex::new(HashMap::new()),
    //     })
    // }

    // pub fn insert<T: 'static + RegistryItem>(&mut self, key: String, value: T) {
    //     let mut data = self.data.lock().unwrap();
    //     data.insert(key, Box::new(value));
    // }

    // pub fn get<T: 'static>(&self, key: &str) -> Option<Arc<T>> {
    //     let data = self.data.lock().unwrap();
    //     return data
    //         .get(key)
    //         .and_then(|item| item.as_any().downcast_ref::<T>())
    //         .map(|value| Arc::new(value.clone()));
    // }

    // pub fn register(&self, key: String, value: T) {
    //     println!("Register Operator {}", key);
    //     let mut data = self.data.lock().unwrap();
    //     data.insert(key, value);
    // }

    // pub fn deregister(&self, key: &str) {
    //     let mut data = self.data.lock().unwrap();
    //     data.remove(key);
    // }

    // pub fn get(&self, key: &str) -> Option<Box<dyn super::template::Operator>> {
    //     let data = self.data.lock().unwrap();
    //     data.get(key).cloned()
    // }
}

pub fn eval() {
    // Create a shared registry
    let registry = Registry::new();

    // Clone the Arc to share the registry
    // let registry_clone = Arc::clone(&registry);

    // Register a key-value pair in the registry
    //registry.register("key1".to_string(), "value1".to_string());

    // Get the value for a key from the registry
    // if let Some(value) = registry.get("key1") {
    //     println!("Got value: {}", value);
    // } else {
    //     println!("Key not found");
    // }

    // Deregister a key from the registry
    // registry.deregister("key1");
}
