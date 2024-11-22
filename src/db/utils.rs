use chrono::NaiveDateTime;

pub fn now() -> NaiveDateTime { chrono::Utc::now().naive_utc() }