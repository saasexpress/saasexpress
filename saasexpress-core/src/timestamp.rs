use chrono::{NaiveDateTime, Utc};

pub trait NaiveDateTimeExt {
    fn to_rfc3339(self) -> String;
}

impl NaiveDateTimeExt for NaiveDateTime {
    fn to_rfc3339(self) -> String {
        self.and_local_timezone(Utc).unwrap().to_rfc3339()
    }
}

pub fn now() -> NaiveDateTime {
    Utc::now().naive_utc()
}
