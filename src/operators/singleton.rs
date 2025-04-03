use std::sync::{Mutex, OnceLock};

pub(crate) struct Singleton {
    pub(crate) value: String,
}

impl Singleton {
    fn new() -> Self {
        Singleton {
            value: "Hello, Singleton!".to_string(),
        }
    }

    pub(super) fn get_value(self) -> String {
        self.value
    }

    pub(super) fn set_value(&mut self, new_value: &str) {
        self.value = new_value.to_string();
    }
}

static INSTANCE: OnceLock<Mutex<Singleton>> = OnceLock::new();

pub(super) fn get_instance() -> &'static Mutex<Singleton> {
    INSTANCE.get_or_init(|| Mutex::new(Singleton::new()))
}
// pub(super) fn get_instance() -> &'static Singleton {
//     INSTANCE.get_or_init(Singleton::new)
// }
