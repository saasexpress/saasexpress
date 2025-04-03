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
}

static INSTANCE: OnceLock<Singleton> = OnceLock::new();

pub(super) fn get_instance_ro() -> &'static Singleton {
    INSTANCE.get_or_init(Singleton::new)
}
