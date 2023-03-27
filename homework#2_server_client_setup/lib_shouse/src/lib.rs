#![feature(associated_type_defaults)]
#![allow(non_camel_case_types)]
#![feature(mutex_unlock)]
#![feature(is_some_and)]
#[allow(unused_imports)]
#[allow(unused_variables)]
#[allow(dead_code)]
#[allow(clippy::manual_map)]
pub mod home;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}
