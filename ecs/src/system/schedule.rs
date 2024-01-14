use std::{fmt::Debug, hash::Hash};

use super::System;

pub trait ScheduleLabel {
    const PLACE: usize;
}

pub struct Startup;

pub struct PreUpdate;

pub struct Update;

pub struct PostUpdate;

impl ScheduleLabel for Startup {
    const PLACE: usize = 0;
}
impl ScheduleLabel for PreUpdate {
    const PLACE: usize = 250;
}
impl ScheduleLabel for Update {
    const PLACE: usize = 500;
}
impl ScheduleLabel for PostUpdate {
    const PLACE: usize = 750;
}

pub const SCHEDULE_MAX_PLACE: usize = 100;
