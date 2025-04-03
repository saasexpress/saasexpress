use std::any::Any;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// trait RegistryItem: Any {
//     fn as_any(&self) -> &dyn Any;
// }

// impl<T: 'static> RegistryItem for T {
//     fn as_any(&self) -> &dyn Any {
//         self
//     }
// }

// Define the struct for the registry
// pub struct Registry {
//     items: Mutex<HashMap<String, Box<dyn super::template::Operator>>>,
// }

// Define a registry to store and manage the variations
pub struct Registry {
    items: HashMap<String, Box<dyn super::operator::Operator>>,
}

// Implement methods for the registry
impl Registry {
    pub fn new() -> Self {
        Registry {
            items: HashMap::new(),
        }
    }

    // pub fn new() -> Arc<Self> {
    //     Arc::new(Self {
    //         items: Mutex::new(HashMap::new()),
    //     })
    // }

    pub fn register(&mut self, item: Box<dyn super::operator::Operator>) {
        println!("Registered {}", item.get_name().to_string());
        self.items.insert(item.get_name().to_string(), item);
    }

    pub fn get(&self, name: &str) -> Option<&Box<dyn super::operator::Operator>> {
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

    // pub fn register(&self, item: Box<dyn super::template::Operator>) {
    //     println!("Register Operator {}", item.get_name().to_string());
    //     let mut items = self.items.lock().unwrap();
    //     items.insert(item.get_name().to_string(), item);
    // }

    // pub fn deregister(&self, key: &str) {
    //     let mut items = self.items.lock().unwrap();
    //     items.remove(key);
    // }

    // pub fn get(&self, key: &str) -> Option<Arc<Box<dyn super::template::Operator>>> {
    //     let items = self.items.lock().unwrap();
    //     items.get(key).map(|item| Arc::new(item.clone()))
    // }
}
