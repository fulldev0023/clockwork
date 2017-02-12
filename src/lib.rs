#![feature(collections_bound)]
#![feature(btree_range)]
#![feature(step_by)]

#![allow(dead_code)]
extern crate chrono;
extern crate nom;

mod time_unit;
mod schedule;

pub use schedule::Schedule;
