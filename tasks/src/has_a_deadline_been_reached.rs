use std::ops::{Add, Sub};
use chrono::{NaiveDate, TimeDelta, Utc};

struct ImportantEvent {
    when: NaiveDate,
}

trait Deadline {
    fn is_passed(&self) -> bool;
}

impl Deadline for ImportantEvent {
    fn is_passed(&self) -> bool {
        Utc::now().date_naive().ge(&self.when)
    }
}

#[test]
fn in_past() {
    let event = ImportantEvent {
        when: Utc::now().date_naive().sub(TimeDelta::hours(25)),
    };

    assert!(event.is_passed())
}

#[test]
fn in_future() {
    let event = ImportantEvent {
        when: Utc::now().date_naive().add(TimeDelta::hours(25)),
    };

    assert!(!event.is_passed())
}
