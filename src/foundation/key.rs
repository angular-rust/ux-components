#![allow(unused_imports)]
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use crate::prelude::BuildWith;

use super::Id;

// println!("uniq i8: {:?}", Key::with(-32 as i8));
// println!("uniq i16: {:?}", Key::with(-5235 as i16));
// println!("uniq i32: {:?}", Key::with(-23512 as i32));
// println!("uniq i64: {:?}", Key::with(-235123 as i64));
// println!("uniq u8: {:?}", Key::with(32 as u8));
// println!("uniq u16: {:?}", Key::with(5235 as u16));
// println!("uniq u32: {:?}", Key::with(23512 as u32));
// println!("uniq u64: {:?}", Key::with(235123 as u64));
// println!("uniq usize: {:?}", Key::with(235123 as usize));
// println!("uniq: {:?}", Key::with("my str key"));
// println!("uniq: {:?}", Key::with(String::from("from String")));

// let base = Key::default();
// println!("BASE: {:?}", base);

// println!("CHILD: {:?}", Key::with(&base));

// let mut hasher = DefaultHasher::new();
// base.hash(&mut hasher);
// println!("Hash is {}", hasher.finish());

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Key(String);

impl Key {
    pub fn id(&self) -> Id {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        Id(hasher.finish())
    }
}
// Creates a unique key whose value does not matter. .
// Can be used in the last mile and as shadowed unique keys
// or as keys that don't matter.
//
impl Default for Key {
    fn default() -> Self {
        Self(format!("default::{:020}", rand::random::<u64>()))
    }
}

// Create a key with a parameter value to use later.
// The developer must make sure that the keys are unique
impl BuildWith<i8> for Key {
    fn with(param: i8) -> Self {
        Self(format!("{:020}", param))
    }
}

// Create a key with a parameter value to use later.
// The developer must make sure that the keys are unique
impl BuildWith<i16> for Key {
    fn with(param: i16) -> Self {
        Self(format!("{:020}", param))
    }
}

// Create a key with a parameter value to use later.
// The developer must make sure that the keys are unique
impl BuildWith<i32> for Key {
    fn with(param: i32) -> Self {
        Self(format!("{:020}", param))
    }
}

// Create a key with a parameter value to use later.
// The developer must make sure that the keys are unique
impl BuildWith<i64> for Key {
    fn with(param: i64) -> Self {
        Self(format!("{:020}", param))
    }
}

// Create a key with a parameter value to use later.
// The developer must make sure that the keys are unique
impl BuildWith<u8> for Key {
    fn with(param: u8) -> Self {
        Self(format!("{:020}", param))
    }
}

// Create a key with a parameter value to use later.
// The developer must make sure that the keys are unique
impl BuildWith<u16> for Key {
    fn with(param: u16) -> Self {
        Self(format!("{:020}", param))
    }
}

// Create a key with a parameter value to use later.
// The developer must make sure that the keys are unique
impl BuildWith<u32> for Key {
    fn with(param: u32) -> Self {
        Self(format!("{:020}", param))
    }
}

// Create a key with a parameter value to use later.
// The developer must make sure that the keys are unique
impl BuildWith<u64> for Key {
    fn with(param: u64) -> Self {
        Self(format!("{:020}", param))
    }
}

// Create a key with a parameter value to use later.
// The developer must make sure that the keys are unique
impl BuildWith<usize> for Key {
    fn with(param: usize) -> Self {
        Self(format!("{:020}", param))
    }
}

// Create a key with a parameter value to use later.
// The developer must make sure that the keys are unique
impl BuildWith<&str> for Key {
    fn with(param: &str) -> Self {
        Self(param.to_owned())
    }
}

// Create a key with a parameter value to use later.
// The developer must make sure that the keys are unique
impl BuildWith<String> for Key {
    fn with(param: String) -> Self {
        Self(param)
    }
}

// Creates a unique key based on the parent key.
// Here we create a key value in the "base::new" format
// to create a unique key from the root of the application.
// In other words, we are setting a parent for the new key.
//
// Therefore, when you are developing reusable composite components,
// it will be convenient to hide the child components and generate a
// unique identifier for them from the keypath.
//
impl BuildWith<Key> for Key {
    fn with(param: Key) -> Self {
        Self(format!("{}::{:020}", &param.0, rand::random::<u64>()))
    }
}

impl BuildWith<&Key> for Key {
    fn with(param: &Key) -> Self {
        Self(format!("{}::{:020}", &param.0, rand::random::<u64>()))
    }
}
